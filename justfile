# brom Task Runner

# Scan for TODOs, FIXMEs, and HACKs across the codebase
scan-todos:
    rg -n -e "TODO" -e "FIXME" -e "HACK" --type-add "code:*.{rs,go,ts,js,svelte,py}" --type code .

# Run coverage analysis (requires cargo-llvm-cov)
coverage:
    cargo llvm-cov --all-features --workspace

# Run coverage analysis and generate HTML report
coverage-html:
    cargo llvm-cov --all-features --workspace --html

# Audit architecture sections
audit-sections:
    rg -n "## " architecture.md

# Log audit: Check log levels counts
logs-level-counts log_dir="logs/":
    rg -c "TRACE|DEBUG|INFO|WARN|ERROR" {{log_dir}}

# Log audit: Get recent warnings/errors
logs-warnings log_dir="logs/":
    rg -n "WARN|ERROR" {{log_dir}} --max-count 50

# Log audit: Track lifecycle events
logs-lifecycle log_dir="logs/":
    rg -n "thread spawned|thread started|thread exiting|initialized|shutting down|dropping" {{log_dir}}

# Log audit: Track connection events
logs-connections log_dir="logs/":
    rg -n "connection established|connection closed|reconnect|evict|cache hit|cache miss" {{log_dir}}

# Log audit: Track latency events
logs-latency log_dir="logs/":
    rg -n "elapsed_ms=|duration_ms=|took [0-9]+ms|latency_ms=" {{log_dir}}

# Log audit: Track latency counts
logs-latency-counts log_dir="logs/":
    rg -c "elapsed_ms=[0-9]+|duration_ms=[0-9]+|took [0-9]+ms" {{log_dir}}

# Log audit: Extract latency values
logs-extract-latency log_dir="logs/":
    rg -o "elapsed_ms=[0-9]+" {{log_dir}} --no-filename

# Security: Scan for secrets
scan-secrets:
    rg -n -i -e "API_KEY\s*=" -e "SECRET\s*=" -e "PASSWORD\s*=" -e "TOKEN\s*=" . --glob "!.git" --glob "!target" --glob "!*.lock"

# Audit: Scan for phase stubs
scan-stubs:
    rg "STUB\(Phase" .

# Docs coverage: public items count (Rust)
stats-public-items src_dir="src/":
    rg -c -e "pub\s+fn\s+" -e "pub\s+struct\s+" -e "pub\s+enum\s+" -e "pub\s+trait\s+" -e "pub\s+type\s+" {{src_dir}} --glob "*.rs"

# Docs coverage: doc comments count (Rust)
stats-doc-comments src_dir="src/":
    rg -c "\s*///" {{src_dir}} --glob "*.rs"

# Display the git diff of the most recent commit
git-diff-last:
    git diff HEAD~1..HEAD

# Check PowerShell version safely
pwsh-version:
    @pwsh -NoProfile -Command 'Write-Output $PSVersionTable.PSVersion.ToString()'

# Verify entire toolchain versioning (avoids chaining operators in shell)
verify-toolchain: pwsh-version
    git --version
    rg --version
    sg --version
    rustc --version
    cargo --version
    cargo clippy --version
    rustfmt --version
