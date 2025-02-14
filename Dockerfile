FROM rust:latest

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y default-mysql-client default-libmysqlclient-dev

RUN cargo build --release
CMD ["./target/release/mixmag_axum"]
