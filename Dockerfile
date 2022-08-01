FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/rust-covid-api
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rust_covid_api /usr/local/bin/rust-covid-api
EXPOSE 8081

CMD ["rust-covid-api"]