FROM rust:1.65.0 as builder

WORKDIR /build

COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
COPY ./src /build/src

RUN cargo build --release

FROM rust:1.65.0 as runner

WORKDIR /app

COPY --from=builder /build/target/release/ /app

ENV TELOXIDE_TOKEN=${TELOXIDE_TOKEN}
ENV RUST_LOG=${RUST_LOG}
ENV PORT=${PORT}
ENV DOMAIN=${DOMAIN}
ENV POLLING_MODE=${POLLING_MODE}

ARG EXPOSE_PORT=${PORT}

EXPOSE ${EXPOSE_PORT}

ENTRYPOINT ["/app/gengo-no-rutsu"]