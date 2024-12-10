# KMOPS: An Operating System

## Getting Started

- Install [cargo-make](https://github.com/sagiegurari/cargo-make)

### Native
1. `cargo make build`
    - Builds and compiles the bootloader and kernel, producing a bootable disk image

### Docker
After installing `cargo-make`, run any of the following Docker tasks:
1. `cargo make docker_build_cache_image`
    - Builds the cached image layer to avoid having to recompile dependencies each time.
    - All subsequent docker tasks depend upon this task!
2. `cargo make docker_build `
    - Builds the kernel image and places it in the host's `out` directory
3. `cargo make docker_test`
   - Builds and run all tests for the kernel inside a Docker container.
4. `cargo make docker_run`
   - Runs the kernel image in QEMU, displaying it in a novnc window in your web browser of choice.
   - When you run this task, you'll see the URL to navigate to displayed in your terminal like so:
```bash
novnc   | Navigate to this URL:
novnc   | 
novnc   |     http://172.18.0.3:6080/vnc.html?host=172.18.0.3&port=6080
```
