#[[IF DOCKER true]]
FROM rust:1.68 AS builder
RUN rustup component add rustfmt

WORKDIR /builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS app
WORKDIR /app
COPY --from=builder /builder/target/release/rust_project_name_t .
CMD ["./rust_project_name_t"]
#[[ENDIF]]
