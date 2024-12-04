day1:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd day1
    cargo build --target wasm32-wasip2
    wasmtime --dir=dat ./target/wasm32-wasip2/debug/aoc_24_day1.wasm

day2:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd day2
    cargo build --target wasm32-wasip2
    wasmtime --dir=dat ./target/wasm32-wasip2/debug/aoc_24_day2.wasm

day3:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd day3
    cargo build --target wasm32-wasip2
    wasmtime --dir=dat ./target/wasm32-wasip2/debug/aoc_24_day3.wasm

day4:
    #!/usr/bin/env bash
    set -euxo pipefail
    cd day4
    cargo build --target wasm32-wasip2
    wasmtime --dir=dat ./target/wasm32-wasip2/debug/aoc_24_day4.wasm
