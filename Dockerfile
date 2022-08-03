FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /rust-covid-api
COPY . .

# Create appuser
ENV USER=rust-covid-api
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

# Import from builder.
COPY --from=build /etc/passwd /etc/passwd
COPY --from=build /etc/group /etc/group

WORKDIR /rust-covid-api

COPY --from=build /usr/local/cargo/bin/rust_covid_api ./
EXPOSE 8082

# Use an unprivileged user.
USER rust-covid-api:rust-covid-api

CMD ["./rust_covid_api"]
