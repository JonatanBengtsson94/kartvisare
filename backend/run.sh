#!/bin/bash

# export environment variables
export DB_HOST="localhost"
export DB_PORT="5432"
export DB_USER="kartvisare"
export DB_PASSWORD="kartvisare"
export DB_NAME="kartvisare"

# Run the Rust application
cargo run
