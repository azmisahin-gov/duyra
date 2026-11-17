#!/usr/bin/env bash
set -Eeuo pipefail

artifact_dir="${GITHUB_WORKSPACE:-$PWD}/artifacts/gorsel-denetim"
app_path="${GITHUB_WORKSPACE:-$PWD}/target/release/duyra"
screenshot_path="$artifact_dir/duyra-ana-ekran.png"
status_path="$artifact_dir/durum.txt"
stdout_path="$artifact_dir/uygulama-stdout.txt"
stderr_path="$artifact_dir/uygulama-stderr.txt"
gl_info_path="$artifact_dir/opengl-bilgisi.txt"

mkdir -p "$artifact_dir"

app_pid=""
result="Görüntü alınamadı."
error_detail=""
window_id=""

cleanup() {
  if [[ -n "$app_pid" ]] && kill -0 "$app_pid" 2>/dev/null; then
    kill "$app_pid" 2>/dev/null || true
    wait "$app_pid" 2>/dev/null || true
  fi

  {
    echo "DUYRA Linux CI görsel denemesi"
    echo "Zaman (UTC): $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "Görüntü sunucusu: ${DISPLAY:-Bilinmiyor}"
    echo "OpenGL yazılım çizicisi: ${GALLIUM_DRIVER:-Varsayılan}"
    echo "OpenGL yazılım modu: ${LIBGL_ALWAYS_SOFTWARE:-Kapalı}"
    echo "Pencere tanıtıcısı: ${window_id:-Bulunamadı}"
    echo "Sonuç: $result"
    echo "Ayrıntı: $error_detail"
  } > "$status_path"
}
trap cleanup EXIT

if [[ ! -x "$app_path" ]]; then
  error_detail="Derlenmiş Linux DUYRA uygulaması bulunamadı: $app_path"
  echo "$error_detail" >&2
  exit 1
fi

export LIBGL_ALWAYS_SOFTWARE=1
export GALLIUM_DRIVER=llvmpipe
export RUST_BACKTRACE=1

if command -v glxinfo >/dev/null 2>&1; then
  glxinfo -B > "$gl_info_path" 2>&1 || true
else
  echo "glxinfo bulunamadı." > "$gl_info_path"
fi

"$app_path" > "$stdout_path" 2> "$stderr_path" &
app_pid=$!

deadline=$((SECONDS + 30))
while (( SECONDS < deadline )); do
  if ! kill -0 "$app_pid" 2>/dev/null; then
    wait "$app_pid" || exit_code=$?
    stdout="$(cat "$stdout_path" 2>/dev/null || true)"
    stderr="$(cat "$stderr_path" 2>/dev/null || true)"
    error_detail="DUYRA pencere açılmadan sonlandı. Çıkış kodu: ${exit_code:-bilinmiyor}. STDERR: $stderr STDOUT: $stdout"
    echo "$error_detail" >&2
    exit 1
  fi

  window_id="$(xdotool search --onlyvisible --name 'DUYRA' 2>/dev/null | head -n 1 || true)"
  if [[ -n "$window_id" ]]; then
    break
  fi
  sleep 0.5
done

if [[ -z "$window_id" ]]; then
  error_detail="30 saniye içinde görünür DUYRA penceresi bulunamadı."
  echo "$error_detail" >&2
  exit 1
fi

sleep 2
geometry="$(xdotool getwindowgeometry --shell "$window_id")"
width="$(awk -F= '/^WIDTH=/{print $2}' <<< "$geometry")"
height="$(awk -F= '/^HEIGHT=/{print $2}' <<< "$geometry")"

if [[ "${width:-0}" -lt 400 || "${height:-0}" -lt 300 ]]; then
  error_detail="DUYRA pencere boyutu beklenenden küçük: ${width:-0}x${height:-0}."
  echo "$error_detail" >&2
  exit 1
fi

import -window "$window_id" "$screenshot_path"

if [[ ! -s "$screenshot_path" ]]; then
  error_detail="Ekran görüntüsü dosyası oluşmadı."
  echo "$error_detail" >&2
  exit 1
fi

result="Başarılı: DUYRA ana ekranı Linux yazılım OpenGL ile görüntülendi."
echo "$result"
