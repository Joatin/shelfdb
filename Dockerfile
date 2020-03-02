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
ARG BUILD_DATE
ARG VCS_REF
ARG VERSION
LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.name="Shelfdb" \
      org.label-schema.description="The GraphQL Database!" \
      org.label-schema.url="https://shelfdb.netlify.com" \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/joatin/shelfdb" \
      org.label-schema.vendor="Joatin Granlund" \
      org.label-schema.version=$VERSION \
      org.label-schema.schema-version="1.0"


COPY --from=build /usr/src/app/target/release/shelf /
ENV SHELF_HOST="0.0.0.0"
EXPOSE 5600
VOLUME .shelf_data
ENTRYPOINT [ "/shelf" ]
