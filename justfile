install_build_dependencies:
    cargo install bootimage

test: install_build_dependencies
    cargo test

build: install_build_dependencies
    cargo bootimage

docker_build_cache_image:
    docker build --platform linux/amd64 -t kmops-cache:latest -f Dockerfile.cache .

docker_test: docker_build_cache_image
    docker run --rm --name kmops -it kmops-cache:latest cargo test

docker_test_lib: docker_build_cache_image
    docker run --rm --name kmops -it kmops-cache:latest cargo test --lib

docker_build: docker_build_cache_image
    mkdir -p out
    docker run --name kmops --platform linux/amd64 -it kmops-cache:latest /bin/sh -c "cargo make build"
    docker cp kmops:/build/target/x86_64-os/debug/bootimage-kmops.bin ./out/bootimage-kmops.bin
    docker rm kmops

docker_run: docker_build_cache_image
    docker-compose build
    docker-compose up --remove-orphans
