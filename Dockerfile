# Latest Rust stable release as the base Image
FROM rust:1.72.0

# Let's switch our working dir to `app` == cd app
# The `app` folder will be created for use by Docker in case it does not
# exits already
WORKDIR /app

# install depends
RUN apt update && apt install lld clang -y

# copy all files from our working dir to docker image
COPY . .

# Lets build the binary
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT ["./target/release/zero2prod"]
