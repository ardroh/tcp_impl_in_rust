#!/usr/bin/env bash

cargo b --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

sudo setcap cap_net_admin=eip ./target/release/tcp_impl_in_rust
./target/release/tcp_impl_in_rust &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid

