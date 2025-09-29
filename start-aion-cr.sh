#!/bin/bash

# AION-CR Production Startup Script
echo "🚀 Starting AION-CR Compliance & Regulations Platform"
echo "=================================================="

# Check if binary exists
if [ ! -f "./target/release/aion-cr" ]; then
    echo "Building AION-CR..."
    cargo build --release
fi

# Initialize configuration if not exists
if [ ! -f "aion.toml" ]; then
    echo "Configuration file not found. Please create aion.toml"
    exit 1
fi

# Create data directory
mkdir -p data logs

# Initialize database if needed
echo "Initializing database..."
./target/release/aion-cr database init --database-url "sqlite:./data/aion_cr.db" || echo "Database already initialized"

# Load standard frameworks
echo "Loading standard compliance frameworks..."
./target/release/aion-cr compliance load-standards || echo "Frameworks already loaded"

# Initialize organization
echo "Initializing organization..."
./target/release/aion-cr init --organization "Yatrogenesis" --sector "technology" --region "global" || echo "Organization already initialized"

# Start the server
echo "Starting AION-CR server..."
echo "Access the API at: http://localhost:8080"
echo "Health check: http://localhost:8080/health"
echo "API docs: http://localhost:8080/api/v1/docs"
echo ""
echo "Press Ctrl+C to stop the server"

RUST_LOG=info ./target/release/aion-cr server --host 0.0.0.0 --port 8080