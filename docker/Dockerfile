#
#
#
FROM debian:bookworm

LABEL maintainer="darryl.west" version="2024.01.20"

RUN apt-get update && apt-get install -y \
    make binutils autoconf automake autotools-dev libtool pkg-config cmake \
    build-essential gcc g++ uuid-dev clang libboost-all-dev libhiredis-dev \
    curl wget jq vim neovim git-core procps iputils-ping

RUN useradd --create-home dpw

ADD install.tgz /

WORKDIR /home/dpw

CMD [ "/home/dpw/.local/bin/udp-server" ]
