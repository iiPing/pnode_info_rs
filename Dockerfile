# debian 11 is bullseye
FROM rust:1.68.2-bullseye AS builder
ARG VERSION

RUN apt update && apt -y install git curl perl

WORKDIR /usr/src/local/

RUN git clone https://github.com/iiPing/pnode_info_rs.git \
 && cd pnode_info_rs \
 && git fetch \
 && git fetch --tags \
 && git checkout $VERSION \
 && mkdir -p distbin


FROM builder as first_staged

WORKDIR /usr/src/local/pnode_info_rs

RUN ./build bin-release --output-dir=/usr/src/local/pnode_info_rs/distbin


# https://github.com/GoogleContainerTools/distroless
FROM gcr.io/distroless/cc-debian11:latest

# Grab cadvisor from the staging directory.
COPY --from=first_staged /usr/src/local/pnode_info_rs/distbin/pnode_info_rs /usr/local/bin/pnode_info_rs


ENTRYPOINT ["pnode_info_rs"]
