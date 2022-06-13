FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=file-scanner
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /file-scanner

COPY ./ .

ENV SQLX_OFFLINE true
RUN cargo build --release

######################
FROM ubuntu:latest as file-scanner

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /file-scanner

# Copy our build
COPY --from=builder /file-scanner/target/release/file-scanner ./
COPY --from=builder /file-scanner/configuration ./configuration

# Use an unprivileged user.
USER file-scanner:file-scanner

EXPOSE 8000
ENV APP_ENVIRONMENT production

CMD ["/file-scanner/file-scanner"]
