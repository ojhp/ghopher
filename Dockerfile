FROM rust:1.50 AS build

WORKDIR /src

RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN touch ./src/main.rs
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libssl1.1 ca-certificates tini && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/local/cargo/bin/ghopher .

USER 1000

ENV GHOPHER_LOG_LEVEL=info

ENTRYPOINT [ "tini", "--" ]
CMD ["./ghopher"]
