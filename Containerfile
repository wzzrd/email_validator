FROM registry.access.redhat.com/ubi9-minimal:9.0.0-1608 as builder
USER root
RUN microdnf -y install glibc-devel openssl-devel gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /app
COPY . .
RUN ~/.cargo/bin/cargo build --release && strip /app/target/release/email_validation_api


FROM registry.access.redhat.com/ubi9-minimal:9.0.0-1608
RUN microdnf -y install openssl
WORKDIR /root/
COPY --from=builder /app/target/release/email_validation_api /usr/local/bin
CMD ["email_validation_api"]
