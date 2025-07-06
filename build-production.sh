#!/bin/bash

echo "🏗️  Building VersionWatch for production..."

# Stop the server if it's running
./stop-server.sh

# Build the React frontend
echo "⚛️  Building React frontend..."
cd frontend
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Frontend build failed"
    exit 1
fi

echo "✅ Frontend build completed"
cd ..

# Build the Rust backend
echo "🦀 Building Rust backend..."
cargo build --release --bin versionwatch-cli

if [ $? -ne 0 ]; then
    echo "❌ Backend build failed"
    exit 1
fi

echo "✅ Backend build completed"

# Create the distribution folder
echo "📦 Creating distribution package..."
mkdir -p dist
cp target/release/versionwatch-cli dist/
cp -r frontend/dist dist/frontend-dist
cp config.yaml dist/
cp README.md dist/

echo "✅ Production build completed!"
echo "📁 Distribution files are in ./dist/"
echo "🚀 To run in production:"
echo "   cd dist && ./versionwatch-cli serve --port 3000 --v2" 