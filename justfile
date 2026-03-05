# faqifai development tasks

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Build the project
build:
    cargo build

# Run all tests
test:
    cargo test

# Run faqifai against the current directory
run *ARGS:
    cargo run -- run {{ARGS}}

# Show status of all FAQ questions
status:
    cargo run -- status

# Wipe all generated output and start fresh
clean:
    Get-ChildItem -Filter *.faq | ForEach-Object { $cfg = Get-Content $_.FullName -Raw; if ($cfg -match 'output\s*=\s*"([^"]+)"') { $f = $Matches[1]; if (Test-Path $f) { Remove-Item -Force $f; Write-Host "Removed $f" } } }
    Write-Host "Cleaned generated output."

# Build + test
check: build test

# Run with verbose logging
run-verbose *ARGS:
    $env:RUST_LOG="debug"; cargo run -- run {{ARGS}}

# Run with trace-level logging (shows all session events)
run-trace *ARGS:
    $env:RUST_LOG="debug,faqifai=trace"; cargo run -- run {{ARGS}}

# Run a single question (concurrency=1, useful for debugging)
run-one *ARGS:
    cargo run -- run --concurrency 1 {{ARGS}}

# Force regenerate all answers, ignoring TTL and hashes
regen:
    cargo run -- run --force

# Wipe docs then regenerate everything
fresh: clean regen
