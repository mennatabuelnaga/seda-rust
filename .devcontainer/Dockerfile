# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM rust:1-${VARIANT}

ARG DEBIAN_FRONTEND=noninteractive

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
RUN apt-get update \
    && apt-get purge -y imagemagick imagemagick-6-common

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME

# Rustfmt with nightly toolchain
RUN rustup install nightly
RUN rustup default nightly
RUN rustup component add rustfmt --toolchain nightly

# WASM dependencies + Rustup
RUN sudo apt-get -y install --no-install-recommends binaryen
RUN rustup target add wasm32-wasi
RUN rustup target add wasm32-unknown-unknown

# Dependencies
RUN sudo apt-get -y install --no-install-recommends protobuf-compiler

# Tools
RUN sudo apt-get -y install --no-install-recommends vim
