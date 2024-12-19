#!/bin/sh

directories=$(find . -type f -name "Cargo.toml" -exec dirname {} \;)
for dir in $directories; do
  echo "Running cargo clean in $dir"
  (cd "$dir" && cargo clean)
done

echo "Finished cleaning all projects."