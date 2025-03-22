ARG IAI_CALLGRIND_CROSS_TARGET=x86_64-unknown-linux-gnu
ARG IAI_CALLGRIND_CROSS_VALGRIND_VERSION=3.23.0

FROM ghcr.io/cross-rs/${IAI_CALLGRIND_CROSS_TARGET}:main
ARG DEBIAN_FRONTEND=noninteractive

ARG IAI_CALLGRIND_CROSS_VALGRIND_VERSION
COPY prepare_valgrind.sh /
RUN /prepare_valgrind.sh

ARG IAI_CALLGRIND_CROSS_VALGRIND_VERSION
ARG IAI_CALLGRIND_CROSS_TARGET
COPY build_valgrind.sh /
RUN /build_valgrind.sh

COPY cleanup_valgrind.sh /
RUN /cleanup_valgrind.sh

ARG IAI_CALLGRIND_CROSS_TARGET
COPY build_custom_image.sh /
RUN /build_custom_image.sh

ARG IAI_CALLGRIND_CROSS_TARGET
ENV IAI_CALLGRIND_CROSS_TARGET=${IAI_CALLGRIND_CROSS_TARGET}
ENV IAI_CALLGRIND_CROSS_VALGRIND_DESTDIR="/target/valgrind/${IAI_CALLGRIND_CROSS_TARGET}"
ENV IAI_CALLGRIND_CROSS_VALGRIND_TEMPDIR="/valgrind/target"

RUN curl -LO https://github.com/iai-callgrind/iai-callgrind/releases/download/v0.14.0/iai-callgrind-runner-v0.14.0-armv7-unknown-linux-gnueabihf.tar.gz && \
    tar -xzvf iai-callgrind-runner-v0.14.0-armv7-unknown-linux-gnueabihf.tar.gz && \
    mv iai-callgrind-runner-v0.14.0-armv7-unknown-linux-gnueabihf/iai-callgrind-runner /usr/local/bin && \
    ls /usr/local/bin && \
    rm iai-callgrind-runner-v0.14.0-armv7-unknown-linux-gnueabihf.tar.gz
