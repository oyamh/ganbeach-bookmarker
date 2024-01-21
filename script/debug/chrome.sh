#!/bin/bash
set -e

cd `dirname $0`
cd ../..
cd ./dev/builder
cargo run -- -b chrome
