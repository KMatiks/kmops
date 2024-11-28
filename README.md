# KMOPS: An Operating System

## Getting Started

- Install [cargo-make](https://github.com/sagiegurari/cargo-make)

### Native

1. `cargo make build`
    - Builds and compiles the bootloader and kernel, producing a bootable disk image

### Docker

After installing `cargo-make`, run the following commands to generate the kernel image in an `out` directory at the root of the repo.
1. `cargo make build_docker_cache_image`
    - Builds the cached image layer to avoid having to recompile dependencies each time.
2. `cargo make build_docker `
    - Builds the kernel image and places it in the host's `out` directory
