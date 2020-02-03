FROM rust:1 as build
WORKDIR /usr/src/app
COPY Cargo.toml .
COPY shelf/ ./shelf
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /usr/src/app/target/release/shelf /
ENTRYPOINT [ "/shelf" ]