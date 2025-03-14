
FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin main



FROM alpine AS runtime
RUN addgroup -S runner && adduser -S runner -G runner

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/main /usr/local/bin/
COPY ./docker_log4rs.toml /etc/log4rs.toml
# NOTE : log4rs 를 위해 추가

RUN mkdir /var/log/main/ && mkdir /var/log/main/rolling/ && chown -R runner:runner /var/log/main

USER runner
CMD ["/usr/local/bin/main"]
