#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <problem_name>"
  exit 1
fi
 
# Find the highest number prefix among directories
latest=$(ls -d */ 2>/dev/null | grep -E '^[0-9]+_' | sed 's/_.*//' | sort -n | tail -1)
next=$((latest + 1))
 
dir="${next}_$1"
mkdir "$dir"
cp "new_template.java" "$dir/Solution.java"
echo "Created: $dir"
