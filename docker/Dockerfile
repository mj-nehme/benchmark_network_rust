FROM rust:latest

ARG VERSION="0.1.0"
ARG AUTHOR="Mohamad-Jaafar NEHME"
ARG EMAIL="mohamad.jaafar.nehme@gmail.com"
ARG WORKSPACE="benchmark_network_rust"
ARG PACKAGE="benchmark_network"
ARG USERNAME="jaafar"
ARG USERID=5000
ARG USER_HOME=/home/${USERNAME}
ARG BASH=/bin/bash
ARG GITHUB_PATH="https://github.com/mj-nehme/${WORKSPACE}.git"

LABEL author=${AUTHOR}
LABEL version=${VERSION}
LABEL platform=${WORKSPACE}

RUN useradd --uid ${USERID} --create-home --shell ${BASH} ${USERNAME}

RUN apt update \
    && apt install -y \
    linux-perf \
    linux-base \
    vim\
    iperf \
    curl \
    build-essential \
    net-tools \
    iputils-ping \
    ca-certificates \
    git \
    openssh-client \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/* \
    && apt upgrade

USER ${USERNAME}
WORKDIR ${USER_HOME}

RUN git clone ${GITHUB_PATH}

WORKDIR "${USER_HOME}/${WORKSPACE}/"

RUN git config --global user.email ${EMAIL}
RUN git config --global user.name ${AUTHOR}
RUN git config pull.rebase false

RUN rustup update
RUN cargo clean
RUN RUSTFLAGS='-C force-frame-pointers=y' cargo build --release --benches --tests