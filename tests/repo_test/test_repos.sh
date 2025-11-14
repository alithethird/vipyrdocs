#!/bin/bash

# Script to test vipyrdocs on multiple repositories
# Usage: ./test_repos.sh

set -e
set -x

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPOS_FILE="${REPOS_FILE:-$SCRIPT_DIR/repos.txt}"
TEST_DIR="$SCRIPT_DIR/downloaded_repos"
VIPYRDOCS_PATH="$SCRIPT_DIR/../../target/release/vipyrdocs"
LOG_FILE="$SCRIPT_DIR/test_results.log"
echo "Repositories will be selected from: $REPOS_FILE"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
TOTAL_REPOS=0
SUCCESSFUL_REPOS=0
FAILED_REPOS=0
SKIPPED_REPOS=0

# Timing arrays
declare -a REPO_TIMES
declare -a REPO_NAMES
declare -a REPO_FILE_COUNTS
declare -a REPO_LINE_COUNTS
TOTAL_VIPYRDOCS_TIME=0

echo "Starting repository testing..."
echo "Results will be logged to: $LOG_FILE"
echo "Repositories will be downloaded to: $TEST_DIR"

# Create test directory
mkdir -p "$TEST_DIR"

# Initialize log file
echo "Repository Testing Results - $(date)" > "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"

# Build vipyrdocs if it doesn't exist
if [ ! -f "$VIPYRDOCS_PATH" ]; then
    echo "Building vipyrdocs..."
    cd "$SCRIPT_DIR/../.."
    cargo build --release
    cd "$SCRIPT_DIR"
fi

# Function to test a single repository
test_repository() {
    local repo_url="$1"
    local repo_name=$(basename "$repo_url")
    local repo_dir="$TEST_DIR/$repo_name"
    
    echo "Testing repository: $repo_url"
    
    # Clone or update repository
    if [ -d "$repo_dir" ]; then
        echo "  Repository already exists, pulling latest..."
        cd "$repo_dir"
        if ! git pull --quiet 2>/dev/null; then
            echo -e "  ${YELLOW}Warning: Failed to update repository${NC}"
        fi
    else
        echo "  Cloning repository..."
        if ! git clone "https://github.com/$repo_url.git" "$repo_dir" --quiet 2>/dev/null; then
            echo -e "  ${RED}Failed to clone repository${NC}"
            echo "FAILED (Clone): $repo_url - Could not clone repository" >> "$LOG_FILE"
            return 1
        fi
        cd "$repo_dir"
    fi
    
    # Find Python files
    local python_files=$(find . -name "*.py" -type f)
    
    if [ -z "$python_files" ]; then
        echo -e "  ${YELLOW}No Python files found, skipping${NC}"
        echo "SKIPPED: $repo_url - No Python files found" >> "$LOG_FILE"
        return 2
    fi
    
    local py_count=$(echo "$python_files" | wc -l)
    
    # Count total lines of Python code
    local total_lines=0
    while IFS= read -r py_file; do
        if [ -f "$py_file" ]; then
            lines=$(wc -l < "$py_file" 2>/dev/null || echo 0)
            total_lines=$((total_lines + lines))
        fi
    done <<< "$python_files"
    
    echo "  Found $py_count Python files ($total_lines lines), running vipyrdocs on repository..."
    
    # Start timing vipyrdocs execution
    local start_time=$(date +%s.%N)
    
    # Run vipyrdocs on the entire repository folder
    local vipyrdocs_output
    local exit_code=0
    
    if vipyrdocs_output=$(timeout 120s "$VIPYRDOCS_PATH" "." 2>&1); then
        exit_code=0
    else
        exit_code=$?
    fi
    
    # End timing vipyrdocs execution
    local end_time=$(date +%s.%N)
    local execution_time=$(awk "BEGIN {print $end_time - $start_time}")
    
    # Store timing and statistics data
    REPO_TIMES+=("$execution_time")
    REPO_NAMES+=("$repo_name")
    REPO_FILE_COUNTS+=("$py_count")
    REPO_LINE_COUNTS+=("$total_lines")
    TOTAL_VIPYRDOCS_TIME=$(awk "BEGIN {print $TOTAL_VIPYRDOCS_TIME + $execution_time}")
    
    if [ $exit_code -eq 0 ]; then
        printf "  ${GREEN}Success: Processed %d files (%d lines) in %.2f seconds${NC}\n" "$py_count" "$total_lines" "$execution_time"
        printf "SUCCESS: %s - Processed %d Python files (%d lines) successfully in %.2f seconds\n" "$repo_url" "$py_count" "$total_lines" "$execution_time" >> "$LOG_FILE"
        return 0
    else
        printf "  ${RED}Failed: vipyrdocs crashed/failed in %.2f seconds (exit code: %d)${NC}\n" "$execution_time" "$exit_code"
        printf "FAILED: %s - vipyrdocs crashed/failed in %.2f seconds (exit code: %d)\n" "$repo_url" "$execution_time" "$exit_code" >> "$LOG_FILE"
        echo "    Error output: $vipyrdocs_output" >> "$LOG_FILE"
        return 1
    fi
}

# Main testing loop
while IFS= read -r repo_url; do
    # Skip empty lines and comments
    if [[ -z "$repo_url" || "$repo_url" =~ ^[[:space:]]*# ]]; then
        continue
    fi
    
    TOTAL_REPOS=$((TOTAL_REPOS + 1))
    
    echo ""
    echo "[$TOTAL_REPOS] Processing: $repo_url"
    
    if test_repository "$repo_url"; then
        SUCCESSFUL_REPOS=$((SUCCESSFUL_REPOS + 1))
    elif [ $? -eq 2 ]; then
        SKIPPED_REPOS=$((SKIPPED_REPOS + 1))
    else
        FAILED_REPOS=$((FAILED_REPOS + 1))
    fi
    
done < "$REPOS_FILE"

# Summary
echo ""
echo "========================================"
echo "Testing Summary:"
echo "Total repositories: $TOTAL_REPOS"
echo -e "Successful: ${GREEN}$SUCCESSFUL_REPOS${NC}"
echo -e "Failed: ${RED}$FAILED_REPOS${NC}"
echo -e "Skipped: ${YELLOW}$SKIPPED_REPOS${NC}"

# Calculate timing statistics
if [ ${#REPO_TIMES[@]} -gt 0 ]; then
    echo ""
    echo "Timing Statistics (vipyrdocs execution only):"
    printf "Total vipyrdocs time: %.2f seconds\n" "$TOTAL_VIPYRDOCS_TIME"
    
    # Calculate average
    avg_time=$(awk "BEGIN {printf \"%.2f\", $TOTAL_VIPYRDOCS_TIME / ${#REPO_TIMES[@]}}")
    printf "Average time per repository: %.2f seconds\n" "$avg_time"
    
    # Find min and max times
    min_time=${REPO_TIMES[0]}
    max_time=${REPO_TIMES[0]}
    min_repo=${REPO_NAMES[0]}
    max_repo=${REPO_NAMES[0]}
    
    for i in "${!REPO_TIMES[@]}"; do
        time=${REPO_TIMES[$i]}
        name=${REPO_NAMES[$i]}
        
        if (( $(awk "BEGIN {print ($time < $min_time)}") )); then
            min_time=$time
            min_repo=$name
        fi
        
        if (( $(awk "BEGIN {print ($time > $max_time)}") )); then
            max_time=$time
            max_repo=$name
        fi
    done
    
    printf "Fastest repository: %s (%.2f seconds)\n" "$min_repo" "$min_time"
    printf "Slowest repository: %s (%.2f seconds)\n" "$max_repo" "$max_time"
    
    # Show top 5 longest running repositories
    echo ""
    echo "Top 5 Longest Running Repositories:"
    echo "=================================================================="
    printf "%-3s %-25s %8s %8s %8s\n" "Pos" "Repository" "Files" "Lines" "Time(s)"
    echo "=================================================================="
    
    # Create a temporary file to sort repositories by time (include all data)
    temp_file=$(mktemp)
    for i in "${!REPO_TIMES[@]}"; do
        printf "%.6f %s %d %d\n" "${REPO_TIMES[$i]}" "${REPO_NAMES[$i]}" "${REPO_FILE_COUNTS[$i]}" "${REPO_LINE_COUNTS[$i]}" >> "$temp_file"
    done
    
    # Sort by time (descending) and show top 5
    counter=0
    sort -nr "$temp_file" | head -5 | while read -r time name files lines; do
        counter=$((counter + 1))
        printf "%2d. %-25s %8d %8d %8.2f\n" "$counter" "$name" "$files" "$lines" "$time"
    done
    
    # Clean up temp file
    rm -f "$temp_file"
fi
echo ""

# Add summary to log file
echo "" >> "$LOG_FILE"
echo "========================================" >> "$LOG_FILE"
echo "Summary:" >> "$LOG_FILE"
echo "Total repositories: $TOTAL_REPOS" >> "$LOG_FILE"
echo "Successful: $SUCCESSFUL_REPOS" >> "$LOG_FILE"
echo "Failed: $FAILED_REPOS" >> "$LOG_FILE"
echo "Skipped: $SKIPPED_REPOS" >> "$LOG_FILE"

# Add timing statistics to log
if [ ${#REPO_TIMES[@]} -gt 0 ]; then
    echo "" >> "$LOG_FILE"
    echo "Timing Statistics:" >> "$LOG_FILE"
    printf "Total vipyrdocs time: %.2f seconds\n" "$TOTAL_VIPYRDOCS_TIME" >> "$LOG_FILE"
    avg_time_log=$(awk "BEGIN {printf \"%.2f\", $TOTAL_VIPYRDOCS_TIME / ${#REPO_TIMES[@]}}")
    printf "Average time per repository: %.2f seconds\n" "$avg_time_log" >> "$LOG_FILE"
    
    echo "" >> "$LOG_FILE"
    echo "Top 5 Longest Running Repositories:" >> "$LOG_FILE"
    echo "==================================================================" >> "$LOG_FILE"
    printf "%-3s %-25s %8s %8s %8s\n" "Pos" "Repository" "Files" "Lines" "Time(s)" >> "$LOG_FILE"
    echo "==================================================================" >> "$LOG_FILE"
    
    # Add top 5 to log file too
    temp_file_log=$(mktemp)
    for i in "${!REPO_TIMES[@]}"; do
        printf "%.6f %s %d %d\n" "${REPO_TIMES[$i]}" "${REPO_NAMES[$i]}" "${REPO_FILE_COUNTS[$i]}" "${REPO_LINE_COUNTS[$i]}" >> "$temp_file_log"
    done
    
    counter_log=0
    sort -nr "$temp_file_log" | head -5 | while read -r time name files lines; do
        counter_log=$((counter_log + 1))
        printf "%2d. %-25s %8d %8d %8.2f\n" "$counter_log" "$name" "$files" "$lines" "$time" >> "$LOG_FILE"
    done
    rm -f "$temp_file_log"
    
    echo "" >> "$LOG_FILE"
    echo "Individual repository details:" >> "$LOG_FILE"
    echo "========================================================================" >> "$LOG_FILE"
    printf "%-30s %8s %8s %8s\n" "Repository" "Files" "Lines" "Time(s)" >> "$LOG_FILE"
    echo "========================================================================" >> "$LOG_FILE"
    for i in "${!REPO_TIMES[@]}"; do
        printf "%-30s %8d %8d %8.2f\n" "${REPO_NAMES[$i]}" "${REPO_FILE_COUNTS[$i]}" "${REPO_LINE_COUNTS[$i]}" "${REPO_TIMES[$i]}" >> "$LOG_FILE"
    done
fi

echo "Test completed at: $(date)" >> "$LOG_FILE"

if [ $FAILED_REPOS -gt 0 ]; then
    echo -e "${RED}Some repositories failed testing. Check $LOG_FILE for details.${NC}"
    exit 1
else
    echo -e "${GREEN}All repositories passed testing!${NC}"
    exit 0
fi