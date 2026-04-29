FROM debian:trixie-20260421

RUN apt-get update && apt-get install -y rustup gcc z3 unzip curl

WORKDIR /app

# Install Liquid Fixpoint (for Flux)
ADD --unpack=true https://github.com/ucsd-progsys/liquid-fixpoint/releases/download/nightly-01-14-2026/fixpoint-x86_64-linux-gnu.tar.gz /opt/liquid-fixpoint
ENV PATH="/opt/liquid-fixpoint:${PATH}"

# Install Flux
ADD https://github.com/flux-rs/flux.git#009f708f63649f2b0af5cc3f9e6792384cf8eed6 /opt/flux
WORKDIR /opt/flux
RUN cargo xtask install

# Install Kani
RUN cargo install --locked kani-verifier --version 0.67.0 && cargo kani setup

# Install Verus
RUN rustup install 1.95.0-x86_64-unknown-linux-gnu
ADD https://github.com/verus-lang/verus/releases/download/release%2F0.2026.04.24.f8e1704/verus-0.2026.04.24.f8e1704-x86-linux.zip verus.zip
RUN unzip verus.zip -d /opt/verus && \
    rm verus.zip
RUN chmod +x /opt/verus/verus-x86-linux/verus
ENV PATH="/opt/verus/verus-x86-linux:${PATH}"
