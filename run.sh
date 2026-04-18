set -e # Exit early if any commands fail
(
  cd "$(dirname "$0")"
  cargo build --release --target-dir=/tmp/codecrafters-build-claude-code-rust --manifest-path Cargo.toml
)
exec /tmp/codecrafters-build-claude-code-rust/release/codecrafters-claude-code "$@"
