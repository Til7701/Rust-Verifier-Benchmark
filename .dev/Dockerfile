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
RUN cargo install --locked kani-verifier --version 0.67.0
RUN cargo kani setup

# Install Prusti
WORKDIR /app
RUN apt-get install -y openjdk-21-jdk
RUN rustup install nightly-2023-08-15-x86_64-unknown-linux-gnu
ADD https://github.com/viperproject/prusti-dev/releases/download/v-2023-08-22-1715/prusti-release-ubuntu.zip prusti.zip
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

# Install Aeneas
#RUN apt-get update && apt-get install -y opam libgmp-dev pkg-config
#RUN opam init --disable-sandboxing -y
#RUN eval $(opam env)
#RUN opam switch create 5.3.0
#RUN opam install -y ppx_deriving visitors easy_logging zarith yojson core_unix odoc \
#    ocamlgraph menhir ocamlformat.0.27.0 unionFind zarith progress domainslib
#RUN opam install -y dune ocaml-lsp-server odoc ocamlformat utop
#ADD https://github.com/AeneasVerif/aeneas.git#a2fcf1923d16684cec638f5503da11acd0ef002d /opt/aeneas
#WORKDIR /opt/aeneas
#RUN make setup-charon
#RUN make
