# Shortner Url

### This project was made with the intention of learning rust with a simple application

## Dependencies:
- MongoDB

## API Endpoints
- `<BASE_URL>/_health`: health check of this app
- `<BASE_URL>/url`: Generate new short url
- `<BASE_URL>/{shortner}`: Redirect to destination

## Dependencies:
- Rust
- Mongo DB

## Start App:

- Configure `.env` file by `.env.example`;
- Start mongodb by `docker-compose`;
- Start app with: `cargo run`

## Build this app(Production Mode):
```bash
cargo build -r --bin shortner-url
```
##### Result of this build inside `./target/release/shortner-url`

## Technical Debts
- Algorithm to generate short-url and avoid conflict;
- Test mongo connection inside health check
- Unit test;
- Configure pipeline with test and linter;
- Make Dockerfile;

###### That's all folks !
