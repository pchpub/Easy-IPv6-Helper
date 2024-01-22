#!/bin/bash

check_docker_installed() {
    docker -v &> /dev/null
    if [ $? -ne  0 ];
    then
        echo "Docker未安装。"
        return 1
    else
        echo "Docker已安装。"
        return 0
    fi
}

install_docker() {
    read -p "是否安装Docker？(y/n): " user_input
    if [[ "$user_input" == "y" ]]; then
        echo "正在安装Docker..."
        curl -fsSL https://get.docker.com | bash -s docker --mirror Aliyun
        if [ $? -eq 0 ]; then
            echo "Docker安装成功。"
        else
            echo "Docker安装失败。"
            exit 1
        fi
    else
        echo "未安装Docker，退出脚本。"
        exit 1
    fi
}

build_project() {
    docker run -i \
        -v $PWD:/workdir \
        -v ~/.cargo/git:/root/.cargo/git \
        -v ~/.cargo/registry:/root/.cargo/registry \
        registry.gitlab.com/rust_musl_docker/image:stable-latest \
        cargo build --profile=fast -vv --target=x86_64-unknown-linux-musl

    if [ $? -ne 0 ]; then
        echo "编译失败。"
        exit 1
    fi

    if [ -f "./pch-easy-ipv6" ]; then
        rm ./pch-easy-ipv6
    fi

    if [ ! -f "./target/x86_64-unknown-linux-musl/fast/pch-easy-ipv6" ]; then
        echo "未找到编译结果。"
        exit 1
    fi

    mv ./target/x86_64-unknown-linux-musl/fast/pch-easy-ipv6 ./pch-easy-ipv6
    if [ ! -f "./pch-easy-ipv6" ]; then
        echo "移动文件失败。"
        exit 1
    fi

    echo "项目构建成功。"
}

check_docker_installed || install_docker
build_project
