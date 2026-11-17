#!/usr/bin/env bash
set -Eeuo pipefail

workspace="${GITHUB_WORKSPACE:-$PWD}"
artifact_dir="$workspace/artifacts/gorsel-denetim"
test_data_home="$artifact_dir/e2e-veri"
records_dir="$test_data_home/duyra/incelemeler"
app_path="$workspace/target/release/duyra"
status_path="$artifact_dir/inceleme-akisi-durum.txt"

mkdir -p "$artifact_dir"
rm -rf "$test_data_home"
mkdir -p "$test_data_home"

export XDG_DATA_HOME="$test_data_home"
export LIBGL_ALWAYS_SOFTWARE=1
export GALLIUM_DRIVER=llvmpipe
export RUST_BACKTRACE=1

app_pid=""
window_id=""
result="Başarısız"
detail=""

cleanup() {
  if [[ -n "$app_pid" ]] && kill -0 "$app_pid" 2>/dev/null; then
    kill "$app_pid" 2>/dev/null || true
    wait "$app_pid" 2>/dev/null || true
  fi

  {
    echo "DUYRA sahte verili inceleme akışı denetimi"
    echo "Zaman (UTC): $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "XDG_DATA_HOME: $XDG_DATA_HOME"
    echo "Kayıt klasörü: $records_dir"
    echo "Pencere tanıtıcısı: ${window_id:-Bulunamadı}"
    echo "Sonuç: $result"
    echo "Ayrıntı: $detail"
  } > "$status_path"
}
trap cleanup EXIT

fail() {
  detail="$1"
  echo "HATA: $detail" >&2
  exit 1
}

if [[ ! -x "$app_path" ]]; then
  fail "Derlenmiş DUYRA uygulaması bulunamadı: $app_path"
fi

launch_app() {
  local run="$1"
  "$app_path" > "$artifact_dir/inceleme-$run-stdout.log" 2> "$artifact_dir/inceleme-$run-stderr.log" &
  app_pid=$!

  local deadline=$((SECONDS + 30))
  window_id=""
  while (( SECONDS < deadline )); do
    if ! kill -0 "$app_pid" 2>/dev/null; then
      wait "$app_pid" || true
      fail "DUYRA $run. açılışta pencere oluşturmadan kapandı. STDERR: $(cat "$artifact_dir/inceleme-$run-stderr.log" 2>/dev/null || true)"
    fi

    window_id="$(xdotool search --onlyvisible --name '^DUYRA$' 2>/dev/null | head -n 1 || true)"
    if [[ -n "$window_id" ]]; then
      break
    fi
    sleep 0.5
  done

  [[ -n "$window_id" ]] || fail "DUYRA $run. açılışta 30 saniye içinde görünür pencere oluşturmadı."
  sleep 1
}

stop_app() {
  if [[ -n "$app_pid" ]] && kill -0 "$app_pid" 2>/dev/null; then
    kill "$app_pid" 2>/dev/null || true
    wait "$app_pid" 2>/dev/null || true
  fi
  app_pid=""
  window_id=""
  sleep 1
}

click_at() {
  local x="$1"
  local y="$2"
  xdotool mousemove --sync --window "$window_id" "$x" "$y"
  xdotool click --clearmodifiers 1
  sleep 1
}

capture() {
  local name="$1"
  sleep 1
  import -window "$window_id" "$artifact_dir/$name"
  [[ -s "$artifact_dir/$name" ]] || fail "Ekran görüntüsü oluşturulamadı: $name"
}

wait_for_record_count() {
  local expected="$1"
  local deadline=$((SECONDS + 15))
  local actual=0

  while (( SECONDS < deadline )); do
    actual=0
    if [[ -d "$records_dir" ]]; then
      actual="$(find "$records_dir" -maxdepth 1 -type f -name '*.json' | wc -l | tr -d ' ')"
    fi
    if [[ "$actual" == "$expected" ]]; then
      return 0
    fi
    sleep 0.25
  done

  fail "Beklenen kayıt sayısı $expected; bulunan kayıt sayısı $actual."
}

validate_records() {
  python3 - "$records_dir" <<'PY'
import json
import pathlib
import sys
import uuid

directory = pathlib.Path(sys.argv[1])
files = sorted(directory.glob("*.json"))
if len(files) != 2:
    raise SystemExit(f"İki kayıt bekleniyordu; {len(files)} bulundu.")

records = [json.loads(path.read_text(encoding="utf-8")) for path in files]
ids = [record["id"] for record in records]
numbers = [record["number"] for record in records]

for record in records:
    uuid.UUID(record["id"])
    if record["schema_version"] != 1:
        raise SystemExit("Beklenmeyen kayıt şema sürümü.")
    if not record["number"].startswith("DYR-"):
        raise SystemExit("İnceleme No beklenen DYR- önekiyle başlamıyor.")
    if record["status"] != "Hazırlanıyor":
        raise SystemExit("Yeni incelemenin başlangıç durumu beklenenden farklı.")
    if not record["created_at_unix"]:
        raise SystemExit("Oluşturulma zamanı boş.")

if len(set(ids)) != 2 or len(set(numbers)) != 2:
    raise SystemExit("İnceleme kimlikleri veya numaraları benzersiz değil.")

print("Kayıt doğrulaması başarılı:")
for record in records:
    print(f"- {record['number']} | {record['status']} | {record['id']}")
PY
}

launch_app "ilk"
capture "inceleme-akisi-01-genel-bakis.png"

# Genel Bakış ekranındaki Yeni İnceleme Başlat düğmesi.
click_at 460 464
wait_for_record_count 1
capture "inceleme-akisi-02-ilk-kayit.png"

# Uygulamayı yeniden başlat; kayıt bellekte değil diskte bulunmalıdır.
stop_app
launch_app "yeniden-acilis"
click_at 110 160
sleep 1
wait_for_record_count 1
capture "inceleme-akisi-03-yeniden-acilan-liste.png"

# İncelemeler ekranının sağ üstündeki Yeni İnceleme düğmesi.
click_at 1090 145
wait_for_record_count 2
capture "inceleme-akisi-04-iki-kayit.png"
validate_records

# Her iki kaydı da Aç ile seç; Açık İnceleme alanı seçimle değişmelidir.
click_at 1100 211
xdotool mousemove --sync --window "$window_id" 800 680
capture "inceleme-akisi-05-ilk-kayit-acik.png"
click_at 1100 252
xdotool mousemove --sync --window "$window_id" 800 680
capture "inceleme-akisi-06-ikinci-kayit-acik.png"

difference="$(compare -metric AE "$artifact_dir/inceleme-akisi-05-ilk-kayit-acik.png" "$artifact_dir/inceleme-akisi-06-ikinci-kayit-acik.png" null: 2>&1 || true)"
if [[ ! "$difference" =~ ^[0-9]+$ ]] || (( difference == 0 )); then
  fail "İki farklı kayıt seçildiğinde açık inceleme ekranı değişmedi."
fi

result="Başarılı"
detail="İki farklı inceleme oluşturuldu; uygulama yeniden başlatıldı; kayıtlar geri yüklendi; iki kayıt da Aç eylemiyle seçildi. Görsel piksel farkı: $difference."
echo "$detail"
