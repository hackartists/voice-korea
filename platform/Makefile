SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local
HOST ?= 0.0.0.0:8080
PROFILE ?= default
RUST_LOG ?= info

ifneq ("$(PROFILE)","default")
	AWS_FLAG += --profile $(PROFILE)
endif

ifeq ("$(ENV)","prod")
	LOG_LEVEL ?= info
	VOICE_KOREA_TABLE_NAME = voice-korea-prod
	BASE_URL = https://voice-korea.biyard.co
	API_URL ?= https://voice-korea-api.dev.biyard.co
endif

ifeq ("$(ENV)","dev")
	VOICE_KOREA_TABLE_NAME = voice-korea-dev
	BASE_URL = https://voice-korea.dev.biyard.co
	API_URL ?= https://voice-korea-api.dev.biyard.co
endif

ifeq ("$(ENV)","local")
	VOICE_KOREA_TABLE_NAME = voice-korea-dev
	BASE_URL = http://localhost:8080
	API_URL ?= http://localhost:3000
endif

API_URL ?= https://voice-korea-api.dev.biyard.co
ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)
AWS_DYNAMODB_KEY ?= "id"
RUSTFLAGS ?= -D warnings
LOG_LEVEL ?= debug

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) LOG_LEVEL=$(LOG_LEVEL) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_DYNAMODB_KEY=${AWS_DYNAMODB_KEY} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} TABLE_NAME=$(VOICE_KOREA_TABLE_NAME) BASE_URL=$(BASE_URL) API_URL=${API_URL} RUSTFLAGS="${RUSTFLAGS}"

setup.tool:
	cargo install dioxus-cli --version 0.6.0-rc.0
	cargo install toml-cli
	npm i -g tailwindcss

run: public/tailwind.css
	$(BUILD_ENV) dx serve -p platform

clean:
	rm -rf dist public/tailwind.css .aws-sam

public/tailwind.css:
	tailwindcss -i ./input.css -o ./public/tailwind.css --minify

build: public/tailwind.css
	$(BUILD_ENV) dx build -p platform --release

.ONESHELL:
build-lambda: clean public/tailwind.css
	$(BUILD_ENV) dx build --release --server-features lambda -p platform

version:
	@echo $(VERSION)
