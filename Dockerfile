FROM kmops-cache:latest

WORKDIR /build
COPY . /build

RUN cargo make build

FROM scratch AS export
COPY --from=0 /build/target/x86_64-os/debug/bootimage-kmops.bin /
