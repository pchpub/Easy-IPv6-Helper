docker run -i \
        -v $PWD:/workdir \
        -v ~/.cargo/git:/root/.cargo/git \
        -v ~/.cargo/registry:/root/.cargo/registry \
        registry.gitlab.com/rust_musl_docker/image:stable-latest \
        cargo build --profile=fast -vv --target=x86_64-unknown-linux-musl
rm ./pch-easy-ipv6
mv ./target/x86_64-unknown-linux-musl/fast/pch-easy-ipv6 ./pch-easy-ipv6