FROM rust:1-alpine AS builder

RUN apk add --no-cache musl-dev sqlite-static openssl-dev openssl-libs-static pkgconf git libpq-dev
ENV SYSROOT=/dummy

ARG HOST
ARG PORT
ARG TURNSTILE_SOLVER_HOST
ARG TURNSTILE_SOLVER_PORT
ARG COBALT_API_URL

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# dump the environment to a .env file
RUN echo "HOST=$HOST" > .env
RUN echo "PORT=$PORT" >> .env
RUN echo "TURNSTILE_SOLVER_HOST=$TURNSTILE_SOLVER_HOST" >> .env
RUN echo "TURNSTILE_SOLVER_PORT=$TURNSTILE_SOLVER_PORT" >> .env
RUN echo "COBALT_API_URL=$COBALT_API_URL" >> .env

RUN cargo build --release

FROM scratch
COPY --from=builder /target/release/cobalt-bypass /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
CMD ["/app"]
