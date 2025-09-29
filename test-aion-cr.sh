#!/bin/bash

# AION-CR Testing Script
echo "🧪 Testing AION-CR Functionality"
echo "================================"

# Test if binary is built
if [ ! -f "./target/release/aion-cr" ]; then
    echo "❌ AION-CR binary not found. Run 'cargo build --release' first."
    exit 1
fi

echo "✅ Binary found"

# Test CLI help
echo ""
echo "📋 Testing CLI help..."
./target/release/aion-cr --help || echo "❌ CLI help failed"

# Test framework listing
echo ""
echo "📋 Testing framework listing..."
./target/release/aion-cr framework list || echo "❌ Framework listing failed"

# Test database status
echo ""
echo "📋 Testing database status..."
./target/release/aion-cr database status || echo "❌ Database status failed"

# Test server health (if running)
echo ""
echo "📋 Testing server health..."
curl -s http://localhost:8080/health || echo "❌ Server not running"

echo ""
echo "🏁 Tests completed"