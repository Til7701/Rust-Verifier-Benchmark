FROM debian:trixie-20260421

RUN apt-get update && apt-get install -y rustup gcc z3

# Install Liquid Fixpoint
ADD --unpack=true https://github.com/ucsd-progsys/liquid-fixpoint/releases/download/nightly-01-14-2026/fixpoint-x86_64-linux-gnu.tar.gz /opt/liquid-fixpoint
ENV PATH="/opt/liquid-fixpoint:${PATH}"

# Install Flux
ADD https://github.com/flux-rs/flux.git#009f708f63649f2b0af5cc3f9e6792384cf8eed6 /opt/flux
WORKDIR /opt/flux
RUN cargo xtask install

RUN rustup default stable
