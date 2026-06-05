FROM mcr.microsoft.com/devcontainers/rust:2-1-trixie

USER root
RUN apt update && \
    apt upgrade -y && \
    apt -y install bat curl fzf lsd ripgrep fd-find \
    build-essential \
    cmake \
    ragel \
    pkg-config \
    libsqlite3-dev \
    libpcap-dev \
    libboost-all-dev

USER vscode
RUN mkdir -p /home/vscode/.devcontainer/
COPY postcreate.sh /home/vscode/.devcontainer/
COPY poststart.sh /home/vscode/.devcontainer/

WORKDIR /workspace
