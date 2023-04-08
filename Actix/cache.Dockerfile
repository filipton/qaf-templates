#[[IF DOCKER true]]
FROM debian:bookworm-slim AS app
WORKDIR /app
COPY ./target/release/rust_project_name_t .
CMD ["./rust_project_name_t"]
#[[ENDIF]]
