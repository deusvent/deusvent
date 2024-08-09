#!/bin/bash
set -euo pipefail

usage() {
  echo "Usage:
    ./run.sh build 
    ./run.sh test   
    ./run.sh deploy [ client-web | api/[lambda-name] | www ]
    ./run.sh lint   
    ./run.sh deps"
  exit 1
}

ACTION=""
if [[ -n "${1:-}" ]]; then ACTION=$1; fi
PARAM=""
if [[ -n "${2:-}" ]]; then PARAM=$2; fi
VERSION=${GITHUB_SHA:-$(date +%s)}

log() { echo "[$(date)] $1"; }

# Install dependencies and required tooling for the development
deps() {
  cargo fetch
}

# Builds everything
build() {
  log "Building all Rust projects"
  cargo build --release --all-features
}

# Run all the tests
test() {
  log "Testing all Rust projects"
  cargo test --release
}

# Run linters and other static checkers
lint() {
  log "Linting all the Rust projects"
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
}

s3_site_sync() {
  local files=$1
  local bucket=$2
  if [ ! -f "$files/index.html" ]; then
    echo "Error: No index.html file found in $files"
    exit 1
  fi
  aws s3 sync "$files" "$bucket" --delete
}

deploy() {
  local service="$1"
  log "Deploying $service"
  if [[ "$service" == "www" ]]; then 
    (cd www && docker run -u "$(id -u):$(id -g)" -v $PWD:/app --workdir /app ghcr.io/getzola/zola:v0.17.1 build)
    s3_site_sync "www/public" "s3://deusvent-site-www"
  elif [[ "$service" == api* ]]; then
    deploy_lambdas "$service"
  else 
    log "Specify what to deploy: api | client-web | www"
  fi
}

deploy_lambdas() {
  local filter="$1"
  for lambda_dir in api/lambda-*/; do
    if [[ "$lambda_dir" == *"$filter"* ]]; then
      local name=$(echo "$lambda_dir" | awk -F'/lambda-|/' '{print $2}') # Extract actual lambda name from the path
      log "Deploying lambda: $name"
      (cd "$lambda_dir" && cargo lambda build --arm64 --release --output-format zip)
      aws lambda update-function-code \
        --function-name "api-$name" \
        --zip-file fileb://./target/lambda/lambda-$name/bootstrap.zip \
        --architectures arm64 \
        --no-cli-pager
    fi
  done

  # lambdas=("health" "delete" "find" "set")
  # log "Buidling all lambdas"
  # for lambda in "${lambdas[@]}"; do
  #   
  # done
  # log "Deploying all lambdas"
  # for lambda in "${lambdas[@]}"; do
  #   
  # done
}


case "$ACTION" in
  "build") build ;;
  "test") test ;;
  "deploy") deploy "$PARAM" ;;
  "deps") deps ;;
  "lint") lint ;;
  "ci") 
    build
    lint 
    test 
    ;;
  *) usage ;;
esac
