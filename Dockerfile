# syntax=docker/dockerfile:1.3.1

FROM ubuntu:latest

RUN apt-get update -y \
    && apt-get install -y software-properties-common \
    && apt-get update -y

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update \
    && apt-get install -y \
        bash-completion \
	curl \
	gcc \
	g++ \
	git \
	libssl-dev \
	make \
        pkg-config \
	sudo \
        tree \
        unzip \
        vim \
        wget

# non-root ubuntu user
ARG USER=ubuntu
ARG GROUP=ubuntu
ARG UID=1000
ARG GID=1000

ENV HOME="/home/$USER" \
    USER=$USER \
    GROUP=$GROUP \
    UID=$UID \
    GID=$GID

WORKDIR $HOME
USER $USER

RUN mkdir $HOME/repo \
	&& mkdir $HOME/.config

# Install Rust using rustup (official method)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path
ENV PATH="$HOME/.cargo/bin:${PATH}"

# Install rust analyzer
RUN mkdir -p $HOME/.local/bin \
    && curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > $HOME/.local/bin/rust-analyzer \
    && chmod +x $HOME/.local/bin/rust-analyzer
ENV PATH="$HOME/.local/bin:${PATH}"

# Install the latest stable noevim release
# Note that I don't really know if this is necessary anymore
ENV NVIM_VERSION="0.11.4"
RUN wget https://github.com/neovim/neovim/releases/download/v$NVIM_VERSION/nvim-linux-x86_64.tar.gz -O /tmp/nvim.tar.gz \
    && tar xzvf /tmp/nvim.tar.gz -C $HOME \
    && ln -s $HOME/nvim-linux-x86_64/bin/nvim $HOME/.local/bin/nvim \
    && rm /tmp/nvim.tar.gz

CMD ["sleep", "infinity"]
