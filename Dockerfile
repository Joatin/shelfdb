FROM rust:1 as build
WORKDIR /usr/src/app

# Create stubs
RUN mkdir -p shelf/src && echo 'fn main() {}' >> shelf/src/main.rs && \
    mkdir -p config/src && touch config/src/lib.rs && \
    mkdir -p database/src && touch database/src/lib.rs && \
    mkdir -p file_store/src && touch file_store/src/lib.rs && \
    mkdir -p memory_cache/src && touch memory_cache/src/lib.rs && \
    mkdir -p server/src && touch server/src/lib.rs

# Copy cargo files with all dependencies
COPY Cargo.toml Cargo.lock ./
COPY shelf/Cargo.toml ./shelf
COPY config/Cargo.toml ./config
COPY database/Cargo.toml ./database
COPY file_store/Cargo.toml ./file_store
COPY memory_cache/Cargo.toml ./memory_cache
COPY server/Cargo.toml ./server

# Build first time then clean everything except deps
RUN cargo build --release
RUN cargo clean -p shelf_config --release
RUN cargo clean -p shelf_database --release
RUN cargo clean -p shelf_file_store --release
RUN cargo clean -p shelf_memory_cache --release
RUN cargo clean -p shelf_server --release

COPY shelf/ ./shelf
COPY config/ ./config
COPY database/ ./database
COPY file_store/ ./file_store
COPY memory_cache/ ./memory_cache
COPY server/ ./server

# Build second time
RUN cargo build -p shelf --release

FROM gcr.io/distroless/cc
ARG BUILD_DATE
ARG VCS_REF
ARG VERSION
MAINTAINER "Joatin Granlund <granlundjoatin@icloud.com>"
LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Shelfdb" \
      org.label-schema.description="The GraphQL Database!" \
      org.label-schema.url="https://shelfdb.netlify.com" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/joatin/shelfdb" \
      org.label-schema.vendor="Joatin Granlund" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0" \
      org.label-schema.build-date=$BUILD_DATE \
      org.opencontainers.image.title="Shelfdb" \
      org.opencontainers.image.description="The GraphQL Database!" \
      org.opencontainers.image.url="https://shelfdb.netlify.com" \
      org.opencontainers.image.revision=$VCS_REF \
      org.opencontainers.image.source="https://github.com/joatin/shelfdb" \
      org.opencontainers.image.vendor="Joatin Granlund" \
      org.opencontainers.image.version=$VERSION \
      org.opencontainers.image.authors="Joatin Granlund <granlundjoatin@icloud.com>" \
      org.opencontainers.image.license="MIT"


COPY --from=build /usr/src/app/target/release/shelf /
ENV SHELF_HOST="0.0.0.0"
EXPOSE 5600
VOLUME .shelf_data
ENTRYPOINT [ "/shelf" ]
