# Voice Korea

## Common Environments
| Name     | Description                                                                    |
|----------|--------------------------------------------------------------------------------|
| RUST_LOG | Logging libraries based on tracing(`debug`, `info`, `error`)                   |
| SERVICE  | Package name to be executed. default is `main-ui`                              |
| DOMAIN   | Base domain name (ex. dev.example.com) will be used to compose signing message |


## Development
### Running API Server(main-api)
- It runs `SERVICE` crate.
  - Default `SERVICE` is `main-ui`.

``` bash
export SERVICE=main-api
export NONCE_LAB_API_TOKEN="TOKEN"
make run
```

### Running Web UI(main-ui)
- It will interact with API server in `dev` environment.
  - If you want to change it, set `API_URL` environment.
- Before running UI, set up firebase configuration

``` bash
export SERVICE=main-ui
export API_URL=http://localhost:3000

make run
```


## Deployment
### Main API

``` bash
export SERVICE=main-api
export ENABLE_S3=false
export ENABLE_FARGATE=false
export ENABLE_DYNAMO=false
export ENABLE_RDS=true
export ENABLE_LAMBDA=true
export DOMAIN=
export DATABASE_TYPE=""
export DATABASE_URL=""

ENV=dev make deploy
```
