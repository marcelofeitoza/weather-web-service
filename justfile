run:
    cargo run

run-container:
    docker compose up -d

build:
    cargo build

build-release:
    cargo build --release

check:
    cargo check

clean:
    cargo clean

test:
    #!/bin/bash
    if [ -d tests ]; then
        cd tests
    fi
    if [ ! -d node_modules ]; then
        npm i
        if [ $? -ne 0 ]; then
            echo "Installation failed. Verify your Node.js installation."
        fi
    fi

    npm test
