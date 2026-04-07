#!/bin/bash

# --- Colors ---
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# --- Default Values ---
CPU_QUOTA=100000
MEM_LIMIT="1G"
CPU_CORE="2"
GOVERNOR_PATH="/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor"

usage() {
    local cpu_percent=$((CPU_QUOTA / 1000))
    echo -e "${BLUE}Usage:${NC} isolate [${CYAN}-c${NC} quota] [${CYAN}-m${NC} memory] [${CYAN}-p${NC} core] -- <command>"
    echo -e ""
    echo -e "${BLUE}Options:${NC}"
    echo -e "  ${CYAN}-c${NC} : CPU quota in μs (100000 = 100%)"
    echo -e "  ${CYAN}-m${NC} : Memory limit (e.g., 512M, 2G)"
    echo -e "  ${CYAN}-p${NC} : CPU Pinning (core index)"
    echo -e ""
    echo -e "${BLUE}Example:${NC} IAI_CALLGRIND_SAVE_SUMMARY=json ./isolate.sh -- cargo bench"
    exit 1
}

# --- Cleanup ---
cleanup() {
    [ -z "$SETUP_DONE" ] && exit
    if [ -n "$ORIGINAL_GOVERNOR" ]; then
        echo "$ORIGINAL_GOVERNOR" | sudo tee $GOVERNOR_PATH > /dev/null
        echo -e "\n${GREEN}[OK]${NC} CPU Governor restored to: $ORIGINAL_GOVERNOR"
    fi
}

while getopts "c:m:p:" opt; do
    case $opt in
        c) CPU_QUOTA=$OPTARG ;;
        m) MEM_LIMIT=$OPTARG ;;
        p) CPU_CORE=$OPTARG ;;
        *) usage ;;
    esac
done
shift $((OPTIND-1))

if [ $# -eq 0 ]; then usage; fi

trap cleanup EXIT
SETUP_DONE=true

# --- Save Governor & Set Performance ---
ORIGINAL_GOVERNOR=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
echo "performance" | sudo tee $GOVERNOR_PATH > /dev/null

# --- Resource Calculations ---
CPU_PCT=$((CPU_QUOTA / 1000))
CURRENT_USER=$(whoami)
CURRENT_GID=$(id -g)
COMMAND_NAME="$1"
COMMAND_PATH=$(type -P "$COMMAND_NAME")

if [ -z "$COMMAND_PATH" ]; then
    echo -e "${RED}[ERROR]${NC} Command '$COMMAND_NAME' not found."
    exit 1
fi

# --- Environment Variable Injection ---
ENV_ARGS=()
for var in $(env | grep -E '^(IAI_|DIVAN_|CARGO_|PATH|HOME|USER)' | cut -d= -f1); do
    ENV_ARGS+=("-E" "$var=${!var}")
done

echo -e "${BLUE}--------------------------------------------------${NC}"
echo -e "${YELLOW}[ISOLATE] RESOURCE SUMMARY${NC}"
echo -e "  - CPU Quota:  ${GREEN}${CPU_PCT}%${NC}"
echo -e "  - RAM Limit:  ${GREEN}$MEM_LIMIT${NC}"
echo -e "  - Affinity:   Core ${GREEN}$CPU_CORE${NC}"
echo -e "  - User:       ${GREEN}$CURRENT_USER${NC}"
echo -e "${BLUE}--------------------------------------------------${NC}"

shift

# --- Execute via systemd-run ---
sudo systemd-run \
    --scope \
    --collect \
    --working-directory="$(pwd)" \
    -p CPUQuota=${CPU_PCT}% \
    -p MemoryMax=$MEM_LIMIT \
    -p AllowedCPUs=$CPU_CORE \
    -p User=$CURRENT_USER \
    -p Group=$CURRENT_GID \
    "${ENV_ARGS[@]}" \
    "$COMMAND_PATH" "$@"
