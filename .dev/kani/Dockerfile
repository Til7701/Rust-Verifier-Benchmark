FROM rust:1.95.0

WORKDIR /app

# Install Kani
RUN cargo install --locked kani-verifier --version 0.67.0 && cargo kani setup
