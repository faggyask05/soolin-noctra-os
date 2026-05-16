#!/usr/bin/env bash

MODEL="${1:-noctra-base}"
PROMPT="${2:-Hi}"

for CTX in 512 1024 2048 4096 8192; do
  echo
  echo "===== TEST MODEL=$MODEL CTX=$CTX ====="

  sudo systemctl restart ollama >/dev/null 2>&1
  sleep 5

  timeout 120s curl -N http://127.0.0.1:11434/api/generate \
    -d "{
      \"model\":\"$MODEL\",
      \"prompt\":\"$PROMPT\",
      \"stream\":true,
      \"options\":{
        \"num_predict\":8,
        \"num_ctx\":$CTX,
        \"num_batch\":32,
        \"num_thread\":2
      }
    }"

  RESULT=$?

  echo
  echo "Exit code: $RESULT"

  if [ "$RESULT" -eq 0 ]; then
    echo "SUCCESS at CTX=$CTX"
    exit 0
  else
    echo "FAILED/TIMEOUT at CTX=$CTX"
  fi
done

echo "No working ctx found."
exit 1
