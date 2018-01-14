FROM rust:1.23.0
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:latest
RUN apt-get update && \
       apt-get install -y \
       libpq5 \
       libsqlite3-0 \
       libmariadbclient18 \
       --no-install-recommends
COPY --from=0 /usr/src/myapp/target/release/schani_library /usr/local/bin
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/schani_library"]
