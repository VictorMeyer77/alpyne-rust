# alpyne-rust

### test

#### Run test

    cargo test -- --test-threads=1

#### Run test with coverage html report

prerequisites

    cargo +stable install cargo-llvm-cov --locked

run

     cargo llvm-cov --html
