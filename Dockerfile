# syntax=docker/dockerfile:1.3.1

FROM dev-environment:latest

# Install Rust using rustup (official method)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path
ENV PATH="$HOME/.cargo/bin:${PATH}"

# Install rust analyzer
RUN mkdir -p $HOME/.local/bin \
    && curl -L https://github.com/rust-lang/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz | gunzip -c - > $HOME/.local/bin/rust-analyzer \
    && chmod +x $HOME/.local/bin/rust-analyzer
ENV PATH="$HOME/.local/bin:${PATH}"
