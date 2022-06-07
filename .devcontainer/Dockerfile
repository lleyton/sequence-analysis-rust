FROM fedora:36

RUN dnf -y install git gcc

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 

ENV PATH="/root/.cargo/bin:$PATH"

RUN rustup install stable