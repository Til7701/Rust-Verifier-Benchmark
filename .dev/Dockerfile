FROM debian:trixie-20260421

RUN apt-get update && apt-get install -y rustup gcc z3 unzip curl \
    openjdk-21-jdk \
    opam autoconf libcairo2-dev libexpat1-dev libgmp-dev libgtk-3-dev libgtksourceview-3.0-dev pkg-config zlib1g-dev \
    elan \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Install Aeneas
RUN opam init --disable-sandboxing -y
RUN eval $(opam env)
RUN opam switch create 5.3.0
RUN opam install -y ppx_deriving visitors easy_logging zarith yojson core_unix odoc \
    ocamlgraph menhir ocamlformat.0.27.0 unionFind zarith progress domainslib
RUN opam install -y dune ocaml-lsp-server odoc ocamlformat utop
ADD https://github.com/AeneasVerif/aeneas.git#a2fcf1923d16684cec638f5503da11acd0ef002d /opt/aeneas
WORKDIR /opt/aeneas
RUN make setup-charon
RUN eval $(opam env) && make
ENV PATH="/opt/aeneas/bin:/opt/aeneas/charon/bin:${PATH}"
RUN elan toolchain install leanprover/lean4:v4.28.0-rc1

# Install Creusot
ADD https://github.com/creusot-rs/creusot.git#v0.11.0 /opt/creusot
WORKDIR /opt/creusot
RUN ./INSTALL

# Install Liquid Fixpoint (for Flux)
ADD --unpack=true https://github.com/ucsd-progsys/liquid-fixpoint/releases/download/nightly-01-14-2026/fixpoint-x86_64-linux-gnu.tar.gz /opt/liquid-fixpoint
ENV PATH="/opt/liquid-fixpoint:${PATH}"

# Install Flux
ADD https://github.com/flux-rs/flux.git#009f708f63649f2b0af5cc3f9e6792384cf8eed6 /opt/flux
WORKDIR /opt/flux
RUN cargo xtask install

# Install Kani
RUN cargo install --locked kani-verifier --version 0.67.0
RUN cargo kani setup

# Install Prusti
#RUN apt-get update && apt-get install -y openssl libssl-dev
#ENV JAVA_HOME=/usr/lib/jvm/java-21-openjdk-amd64
#ADD https://github.com/viperproject/prusti-dev.git /opt/prusti
#WORKDIR /opt/prusti
#RUN rm rust-toolchain
#RUN touch rust-toolchain
#RUN echo "[toolchain]" >> rust-toolchain
#RUN echo "channel = \"nightly-2024-01-15\""  >> rust-toolchain
#RUN echo "components = [ \"rustc-dev\", \"llvm-tools-preview\", \"rust-std\", \"rustfmt\", \"clippy\" ]"  >> rust-toolchain
#RUN echo "profile = \"minimal\""  >> rust-toolchain
#RUN ./x.py setup
#RUN ./x.py build --release

RUN rustup install nightly-2023-09-15
ADD https://github.com/viperproject/prusti-dev/releases/download/v-2024-03-26-1504/prusti-release-ubuntu.zip prusti.zip
RUN unzip prusti.zip -d /opt/prusti && rm prusti.zip
RUN chmod +x /opt/prusti/cargo-prusti
RUN chmod +x /opt/prusti/prusti-rustc
RUN chmod +x /opt/prusti/prusti-driver
RUN chmod +x /opt/prusti/viper_tools/z3/bin/z3
ENV PATH="/opt/prusti:${PATH}"

# Install VeriFast
ADD --unpack=true https://github.com/verifast/verifast/releases/download/26.01/verifast-26.01-linux.tar.gz /opt/verifast
ENV PATH="/opt/verifast/verifast-26.01/bin:${PATH}"

# Install Verus
WORKDIR /app
RUN rustup install 1.95.0-x86_64-unknown-linux-gnu
ADD https://github.com/verus-lang/verus/releases/download/release%2F0.2026.04.24.f8e1704/verus-0.2026.04.24.f8e1704-x86-linux.zip verus.zip
RUN unzip verus.zip -d /opt/verus && rm verus.zip
RUN chmod +x /opt/verus/verus-x86-linux/verus
ENV PATH="/opt/verus/verus-x86-linux:${PATH}"
