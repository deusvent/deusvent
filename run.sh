#!/bin/bash
set -euo pipefail

usage() {
  echo "Usage:
    ./run.sh build [ unreal-client ]
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
  rustup target add x86_64-unknown-linux-gnu \
                    aarch64-apple-ios \
                    aarch64-apple-darwin \
                    aarch64-linux-android
  cargo fetch
  cargo install uniffi-bindgen-cpp --git https://github.com/NordSecurity/uniffi-bindgen-cpp --tag v0.6.2+v0.25.0
}

# Builds everything
build() {
  local target="$1"
  
  log "Validating Terraform files"
  if [ ! -d "infra/.terraform" ] && [ -n "$CI" ]; then
    # To run validation we need to init Terraform, but with no backend as state is not accessible from the CI
    (cd infra && terraform init -backend=false)
  fi
  (cd infra && terraform validate)
  
  log "Building all Rust projects"
  cargo build --release --all-features
  
  log "Building logic"
  (cd logic-binding-cpp && ./gen.sh)
  
  if [[ "$target" == "client-unreal" ]]; then
    # Build client-unreal using Docker containers with linux. Read here how to get access and tokens for yourself:
    # https://dev.epicgames.com/documentation/en-us/unreal-engine/container-deployments-and-images-for-unreal-editor-and-unreal-engine
    echo $GHCR_TOKEN | docker login ghcr.io -u $GHCR_TOKEN_USER --password-stdin
    commands=$(cat <<'EOF'
      sudo chown -R $(id -u):$(id -g) /src/client-unreal/deusvent && \
      sudo apt-get update && \
      sudo apt-get install libsqlite3-dev && \
      /home/ue4/UnrealEngine/Engine/Build/BatchFiles/RunUAT.sh BuildCookRun \
        -platform=Linux \
        -clientconfig=Development \
        -serverconfig=Development \
        -project=$PWD/deusvent.uproject \
        -noP4 \
        -nodebuginfo \
        -allmaps \
        -cook \
        -build \
        -stage \
        -prereqs \
        -pak \
        -archive \
        -archivedirectory=/tmp
EOF
)
    docker run --volume $PWD:/src \
               --workdir /src/client-unreal/deusvent \
               --rm \
               ghcr.io/epicgames/unreal-engine:dev-slim-5.4.3 bash -c "$commands"
  fi
}

# Run all the tests
test() {
  log "Testing all Rust projects"
  cargo test --release -- --nocapture
}

# Run linters and other static checkers
lint() {
  log "Linting all the Rust projects"
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  
  log "Linting C++ files"
  find client-unreal -name "*.cpp" -o -name "*.h" \
    | grep -v "Intermediate/Build" \
    | xargs clang-format --Werror -style=file -dry-run
  
  log "Linting Terraform files"
  (cd infra && terraform fmt -check -recursive)
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
    (cd www && docker run --rm -u "$(id -u):$(id -g)" -v $PWD:/app --workdir /app ghcr.io/getzola/zola:v0.19.2 build)
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
}


case "$ACTION" in
  "build") build "$PARAM" ;;
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
