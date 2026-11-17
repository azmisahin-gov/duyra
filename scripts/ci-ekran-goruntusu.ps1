$ErrorActionPreference = "Stop"

$artifactDirectory = Join-Path $env:GITHUB_WORKSPACE "artifacts/gorsel-denetim"
New-Item -ItemType Directory -Path $artifactDirectory -Force | Out-Null

$statusPath = Join-Path $artifactDirectory "durum.txt"
$screenshotPath = Join-Path $artifactDirectory "duyra-ana-ekran.png"
$stdoutPath = Join-Path $artifactDirectory "uygulama-stdout.txt"
$stderrPath = Join-Path $artifactDirectory "uygulama-stderr.txt"
$appPath = Join-Path $env:GITHUB_WORKSPACE "target/release/duyra.exe"

$application = $null
$result = "Görüntü alınamadı."
$windowTitle = ""
$windowHandleValue = "0"
$applicationSession = "Bilinmiyor"
$runnerSession = [System.Diagnostics.Process]::GetCurrentProcess().SessionId
$errorDetail = ""
$graphicsAdapters = "Alınamadı"
$processStartedAt = [DateTime]::UtcNow

try {
    $graphicsAdapters = (Get-CimInstance Win32_VideoController -ErrorAction Stop |
        Select-Object -ExpandProperty Name) -join "; "
}
catch {
    $graphicsAdapters = "Grafik aygıtı sorgulanamadı: $($_.Exception.Message)"
}

$nativeCode = @"
using System;
using System.Runtime.InteropServices;

public static class DuyraCaptureNative
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Rect
    {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool GetWindowRect(IntPtr hWnd, out Rect rect);

    [DllImport("user32.dll")]
    public static extern bool IsWindowVisible(IntPtr hWnd);

    [DllImport("user32.dll")]
    public static extern bool SetForegroundWindow(IntPtr hWnd);
}
"@

try {
    if (-not (Test-Path $appPath)) {
        throw "Derlenmiş DUYRA uygulaması bulunamadı."
    }

    Add-Type -TypeDefinition $nativeCode -Language CSharp
    Add-Type -AssemblyName System.Drawing

    $env:RUST_BACKTRACE = "1"
    $application = Start-Process -FilePath $appPath -PassThru -RedirectStandardOutput $stdoutPath -RedirectStandardError $stderrPath
    $deadline = (Get-Date).AddSeconds(30)
    $windowHandle = [IntPtr]::Zero

    while ((Get-Date) -lt $deadline) {
        $application.Refresh()

        if ($application.HasExited) {
            $stdout = if (Test-Path $stdoutPath) { Get-Content $stdoutPath -Raw } else { "" }
            $stderr = if (Test-Path $stderrPath) { Get-Content $stderrPath -Raw } else { "" }
            throw "DUYRA pencere açılmadan sonlandı. Çıkış kodu: $($application.ExitCode). STDERR: $stderr STDOUT: $stdout"
        }

        $windowHandle = $application.MainWindowHandle
        if ($windowHandle -ne [IntPtr]::Zero) {
            $windowTitle = $application.MainWindowTitle
            break
        }

        Start-Sleep -Milliseconds 500
    }

    if ($windowHandle -eq [IntPtr]::Zero) {
        throw "30 saniye içinde DUYRA ana penceresi bulunamadı. Windows CI oturumunda görünür masaüstü olmayabilir."
    }

    $windowHandleValue = $windowHandle.ToString()
    $applicationSession = $application.SessionId.ToString()

    [void][DuyraCaptureNative]::SetForegroundWindow($windowHandle)
    Start-Sleep -Seconds 2

    $rectangle = New-Object DuyraCaptureNative+Rect
    if (-not [DuyraCaptureNative]::GetWindowRect($windowHandle, [ref]$rectangle)) {
        throw "DUYRA pencere sınırları okunamadı."
    }

    $width = $rectangle.Right - $rectangle.Left
    $height = $rectangle.Bottom - $rectangle.Top
    if ($width -lt 400 -or $height -lt 300) {
        throw "DUYRA pencere boyutu beklenenden küçük: ${width}x${height}."
    }

    $bitmap = [System.Drawing.Bitmap]::new($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    try {
        $graphics.CopyFromScreen(
            $rectangle.Left,
            $rectangle.Top,
            0,
            0,
            $bitmap.Size,
            [System.Drawing.CopyPixelOperation]::SourceCopy
        )
        $bitmap.Save($screenshotPath, [System.Drawing.Imaging.ImageFormat]::Png)
    }
    finally {
        $graphics.Dispose()
        $bitmap.Dispose()
    }

    if (-not (Test-Path $screenshotPath) -or (Get-Item $screenshotPath).Length -lt 10000) {
        throw "Ekran görüntüsü dosyası oluşmadı veya beklenenden küçük."
    }

    $result = "Başarılı: DUYRA ana pencere görüntüsü alındı."
    Write-Host $result
}
catch {
    $errorDetail = $_.Exception.Message
    Write-Error $errorDetail
}
finally {
    @(
        "DUYRA Windows CI görsel denemesi"
        "Zaman (UTC): $([DateTime]::UtcNow.ToString('o'))"
        "Runner oturum numarası: $runnerSession"
        "Grafik bağdaştırıcıları: $graphicsAdapters"
        "Uygulama başlatma zamanı (UTC): $($processStartedAt.ToString('o'))"
        "Uygulama oturum numarası: $applicationSession"
        "Uygulama pencere başlığı: $windowTitle"
        "Pencere tanıtıcısı: $windowHandleValue"
        "Sonuç: $result"
        "Ayrıntı: $errorDetail"
    ) | Set-Content -Path $statusPath -Encoding utf8

    if ($null -ne $application) {
        $application.Refresh()
        if (-not $application.HasExited) {
            Stop-Process -Id $application.Id -Force -ErrorAction SilentlyContinue
        }
    }
}

if ($result -notlike "Başarılı:*") {
    exit 1
}
