FROM rust:1.59.0 

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install --yes build-essential cmake
RUN cargo build --release

ENTRYPOINT ["/app/target/release/zmcat"]