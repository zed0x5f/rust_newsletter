FROM lukemathwalker/cargo-chef as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Latest Rust stable release as the base Image
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /app
# Install openssl - it is dynamicaly linked by some of our dependencies
# Install ca-certs - it is needed to verify tls
# when establishing HTTPS connection
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration  configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
