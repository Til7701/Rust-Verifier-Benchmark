FROM rust:1.95.0

RUN apt-get update && apt-get install -y \
    curl \
    unzip

WORKDIR /app

RUN curl -L -o verus.zip https://github.com/verus-lang/verus/releases/download/release%2F0.2026.04.24.f8e1704/verus-0.2026.04.24.f8e1704-x86-linux.zip && \
    unzip verus.zip -d /opt/verus && \
    rm verus.zip

RUN chmod +x /opt/verus/verus-x86-linux/verus

ENV PATH="/opt/verus/verus-x86-linux:${PATH}"
