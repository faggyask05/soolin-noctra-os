#!/usr/bin/env bash
set -e

CORE_FILE="personality/core/noctra_base.txt"
MODEL_NAME="noctra-base"
MODELFILE="Modelfile.base"

cat > "$MODELFILE" <<MODEL_EOF
FROM llama3.2:3b

PARAMETER num_ctx 1024
PARAMETER num_predict 96
PARAMETER num_batch 128
PARAMETER num_thread 4
PARAMETER temperature 0.65
PARAMETER top_p 0.85

SYSTEM """
$(cat "$CORE_FILE")

RESPONSE DIRECTION:
Magyarul válaszolj.
Röviden, természetesen, karakteresen válaszolj.
Ne túlmagyarázz.
Ne legyél steril súgóablak.
Maradj Noctra.
"""
MODEL_EOF

ollama create "$MODEL_NAME" -f "$MODELFILE"

echo "[NOCTRA MODEL] Built: $MODEL_NAME"
