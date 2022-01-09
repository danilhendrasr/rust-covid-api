FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/nodeflux-assignment
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/nodeflux-assignment /usr/local/bin/nodeflux-assignment
EXPOSE 8081

CMD ["nodeflux-assignment"]