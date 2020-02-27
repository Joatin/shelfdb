FROM rust:1 as build
WORKDIR /usr/src/app
COPY Cargo.toml .
COPY shelf/ ./shelf
COPY config/ ./config
COPY database/ ./database
COPY file_store/ ./file_store
COPY memory_cache/ ./memory_cache
COPY server/ ./server
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /usr/src/app/target/release/shelf /
ENV SHELF_HOST="0.0.0.0"
EXPOSE 5600
VOLUME .shelf_data
ENTRYPOINT [ "/shelf" ]
