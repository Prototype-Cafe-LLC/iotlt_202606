FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

# 常駐ユーザーは vscode（ベース由来）。root でシェルを開かない。
USER vscode

# パッケージは devcontainer 共通の nopasswd sudo のみ短期利用（開発用イメージの想定）。
RUN sudo apt-get update \
    && sudo DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        cmake \
        libudev-dev \
        pkg-config \
        usbutils \
    && sudo rm -rf /var/lib/apt/lists/*

# rustup / cargo は vscode で実行（ベースの CARGO_HOME 配下にキャッシュ／bin が入る）。
RUN rustup target add thumbv6m-none-eabi \
    && cargo install elf2uf2-rs --locked

WORKDIR /workspace
