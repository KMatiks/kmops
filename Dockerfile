FROM kmops-cache:latest

WORKDIR /build
COPY . /build

RUN cargo make build
RUN cargo test

FROM kmops-cache:latest AS export
COPY --from=0 /build/target/x86_64-os/debug/bootimage-kmops.bin /

CMD ["qemu-system-x86_64", "-drive", "format=raw,file=out/bootimage-kmops.bin"]
