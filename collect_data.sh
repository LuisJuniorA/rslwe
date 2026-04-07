#!/bin/bash

# Default Iterations
K_IT=100
E_IT=1000
D_IT=1000

# Resource Configuration
CPU_LIMIT="100000"
MEM_LIMIT="1G"
CPU_CORE="2"

DATA_DIR="docs/data"
REPORT_DIR="docs/reports/criterion"

# Parse arguments: -k, -e, -d
while getopts "k:e:d:" opt; do
    case $opt in
        k) K_IT=$OPTARG ;;
        e) E_IT=$OPTARG ;;
        d) D_IT=$OPTARG ;;
    esac
done

[ -d "target" ] && sudo chown -R $USER:$USER target
rm -rf "$DATA_DIR" "$REPORT_DIR"
mkdir -p "$DATA_DIR" "$REPORT_DIR"

CPU_MODEL=$(lscpu | grep "Model name" | cut -d':' -f2 | xargs)
cat <<EOF > "$DATA_DIR/env.json"
{
    "cpu": "$CPU_MODEL",
    "cpu_limit": "$((CPU_LIMIT / 1000))%",
    "ram_limit": "$MEM_LIMIT",
    "core_affinity": "$CPU_CORE",
    "date": "$(date '+%Y-%m-%d %H:%M:%S')",
    "iterations": {
        "keygen": $K_IT,
        "encrypt": $E_IT,
        "decrypt": $D_IT
    }
}
EOF

# --- 1. IAI-CALLGRIND ---
echo "🚀 Running IAI (K:$K_IT, E:$E_IT, D:$D_IT)..."

export IAI_KEYGEN_ITERS=$K_IT
export IAI_ENCRYPT_ITERS=$E_IT
export IAI_DECRYPT_ITERS=$D_IT
export IAI_CALLGRIND_SAVE_SUMMARY=json

touch benches/iai_lwe.rs

./isolate.sh -c "$CPU_LIMIT" -m "$MEM_LIMIT" -p "$CPU_CORE" -- cargo bench --bench iai_lwe

sudo chown -R $USER:$USER target

find target/iai -name "summary.json" | while read -r file; do
    bench_full_name=$(basename "$(dirname "$file")")
    cp "$file" "$DATA_DIR/iai_${bench_full_name}.json"
done

# --- 2. CRITERION ---
echo "Running Criterion..."
./isolate.sh -c "$CPU_LIMIT" -m "$MEM_LIMIT" -p "$CPU_CORE" -- \
    cargo bench --bench criterion_lwe

sudo chown -R $USER:$USER target

find target/criterion -name "estimates.json" -path "*/new/*" | while read -r file; do
    bench_name=$(basename "$(dirname "$(dirname "$file")")")
    cp "$file" "$DATA_DIR/criterion_${bench_name}.json"
done

# --- 3. SYNC REPORTS ---
echo "Syncing reports..."

rm -rf "$REPORT_DIR"/*
mkdir -p "$REPORT_DIR"
cp -r target/criterion/* "$REPORT_DIR/"

sudo chown -R $USER:$USER .
echo "✅ Done."
