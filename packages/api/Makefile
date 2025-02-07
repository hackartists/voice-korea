ENV ?= local
PORT ?= 3000
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
AUTH_TYPE = jwt
JWT_SECRET ?= "JWT_SECRET"
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)
WORKSPACE_ROOT ?= $(PWD)/../..
RUSTFLAGS ?= -D warnings

ifeq ("$(ENV)","prod")
	RUST_LOG ?= error
endif

ifeq ("$(ENV)","local")
	RUST_LOG ?= debug
endif

NONCE_LAB_API_ENDPOINT = "http://15.165.76.144:7878"
RUST_LOG ?= debug
CORS_ORIGIN ?= "http://localhost:8080"

BUILD_ENV ?= ENV=$(ENV) PORT=$(PORT) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) JWT_SECRET=$(JWT_SECRET) NONCE_LAB_API_ENDPOINT=${NONCE_LAB_API_ENDPOINT} NONCE_LAB_API_TOKEN=${NONCE_LAB_API_TOKEN} AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) RUSTFLAGS="$(RUSTFLAGS)" AUTH_TYPE=$(AUTH_TYPE) CORS_ORIGIN="$(CORS_ORIGIN)"

setup.tool:
	cargo install cargo-watch toml-cli

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(WORKSPACE_ROOT)/target/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap
