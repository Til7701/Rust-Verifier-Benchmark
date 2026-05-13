FROM debian:trixie-20260505-slim AS base

RUN apt-get update && apt-get install -y rustup z3 unzip curl \
    openjdk-21-jdk-headless \
    elan \
    && rm -rf /var/lib/apt/lists/*
ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
RUN rustup toolchain install nightly-2026-02-07 --profile minimal # Aeneas
RUN rustup toolchain install nightly-2026-02-27 --profile minimal # Creusot
RUN rustup toolchain install nightly-2025-11-25 --profile minimal # Flux
RUN rustup toolchain install 1.95.0 --profile minimal # Kani
RUN rustup toolchain install 1.95.0 --profile minimal # VeriFast
RUN rustup toolchain install 1.95.0 --profile minimal # Verus

FROM base AS opam-base

RUN apt-get update && apt-get install -y opam \
    && rm -rf /var/lib/apt/lists/*

FROM base AS build-base

RUN apt-get update && apt-get install -y gcc \
    && rm -rf /var/lib/apt/lists/*

FROM opam-base AS opam-build-base

RUN apt-get update && apt-get install -y libgmp-dev \
    && rm -rf /var/lib/apt/lists/*
RUN opam init --disable-sandboxing -y
RUN opam switch create 5.3.0
RUN opam install -y ppx_deriving visitors easy_logging zarith yojson core_unix odoc \
    ocamlgraph menhir ocamlformat.0.27.0 unionFind zarith progress domainslib \
    dune ocaml-lsp-server odoc ocamlformat utop \
    && opam clean

FROM opam-build-base AS aeneas-build

ADD https://github.com/AeneasVerif/aeneas.git#a2fcf1923d16684cec638f5503da11acd0ef002d /opt/aeneas
WORKDIR /opt/aeneas
RUN make setup-charon \
    && eval $(opam env) \
    && make \
    && rm -r /opt/aeneas/charon/charon

FROM opam-build-base AS creusot-build

RUN apt-get update && apt-get install -y \
    autoconf libcairo2-dev libexpat1-dev libgtk-3-dev libgtksourceview-3.0-dev pkg-config zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*
ADD https://github.com/creusot-rs/creusot.git#v0.11.0 /opt/creusot
WORKDIR /opt/creusot
RUN ./INSTALL \
    && rm -r /opt/creusot \
    && rm -r /root/.local/share/creusot/_opam/.opam-switch

FROM build-base AS flux-build

ADD https://github.com/flux-rs/flux.git#009f708f63649f2b0af5cc3f9e6792384cf8eed6 /opt/flux
WORKDIR /opt/flux
RUN cargo xtask install \
    && rm -r /opt/flux

FROM build-base AS kani-build

RUN cargo install --locked kani-verifier --version 0.67.0
RUN cargo kani setup

FROM opam-base

WORKDIR /app

# Install Aeneas
RUN elan toolchain install leanprover/lean4:v4.28.0-rc1
COPY --from=aeneas-build /opt/aeneas/bin /opt/aeneas/bin
COPY --from=aeneas-build /opt/aeneas/charon/bin /opt/aeneas/charon/bin
COPY --from=aeneas-build /opt/aeneas/backends/lean /opt/aeneas/backends/lean
ENV PATH="/opt/aeneas/bin:/opt/aeneas/charon/bin:${PATH}"

# Install Creusot
COPY --from=creusot-build /root/.cargo/bin/cargo-creusot /root/.cargo/bin/cargo-creusot
COPY --from=creusot-build /root/.local/share/creusot /root/.local/share/creusot

# Install Flux
ADD --unpack=true https://github.com/ucsd-progsys/liquid-fixpoint/releases/download/nightly-01-14-2026/fixpoint-x86_64-linux-gnu.tar.gz /opt/liquid-fixpoint
ENV PATH="/opt/liquid-fixpoint:${PATH}"
COPY --from=flux-build /root/.cargo/bin/flux /root/.cargo/bin/flux
COPY --from=flux-build /root/.cargo/bin/cargo-flux /root/.cargo/bin/cargo-flux
COPY --from=flux-build /root/.flux /root/.flux

# Install Kani
COPY --from=kani-build /root/.cargo/bin/kani /root/.cargo/bin/kani
COPY --from=kani-build /root/.cargo/bin/cargo-kani /root/.cargo/bin/cargo-kani
COPY --from=kani-build /root/.kani /root/.kani

# Install Prusti
#RUN apt-get update && apt-get install -y openssl libssl-dev
#ADD https://github.com/viperproject/prusti-dev.git#auto-update-nightly-2024-01-15 /opt/prusti
#WORKDIR /opt/prusti
#RUN ./x.py setup
#RUN ./x.py build --release --locked

#RUN rustup install nightly-2023-09-15
#ADD https://github.com/viperproject/prusti-dev/releases/download/v-2024-03-26-1504/prusti-release-ubuntu.zip prusti.zip
#RUN unzip prusti.zip -d /opt/prusti && rm prusti.zip
#RUN chmod +x /opt/prusti/cargo-prusti
#RUN chmod +x /opt/prusti/prusti-rustc
#RUN chmod +x /opt/prusti/prusti-driver
#RUN chmod +x /opt/prusti/viper_tools/z3/bin/z3
#ENV PATH="/opt/prusti:${PATH}"

# Install VeriFast
ADD --unpack=true https://github.com/verifast/verifast/releases/download/26.01/verifast-26.01-linux.tar.gz /opt/verifast
ENV PATH="/opt/verifast/verifast-26.01/bin:${PATH}"

# Install Verus
WORKDIR /app
ADD https://github.com/verus-lang/verus/releases/download/release%2F0.2026.04.24.f8e1704/verus-0.2026.04.24.f8e1704-x86-linux.zip verus.zip
RUN unzip verus.zip -d /opt/verus && rm verus.zip
RUN chmod +x /opt/verus/verus-x86-linux/verus
ENV PATH="/opt/verus/verus-x86-linux:${PATH}"
