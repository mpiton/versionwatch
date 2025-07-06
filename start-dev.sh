#!/bin/bash

# Script pour démarrer VersionWatch en mode développement

echo "🚀 Starting VersionWatch Development Environment"
echo

# Build frontend
echo "📦 Building frontend..."
cd frontend && npm install && npm run build
if [ $? -eq 0 ]; then
    echo "✅ Frontend built successfully"
else
    echo "❌ Frontend build failed"
    exit 1
fi

# Start backend
echo "🔧 Starting backend server..."
cd ..
cargo run --bin versionwatch-cli -- serve --port 3000

echo "🎉 Development environment ready!"
echo "🌐 Dashboard available at: http://127.0.0.1:3000"
echo "📊 Metrics API at: http://127.0.0.1:3000/api/metrics"
echo "🔍 Health check at: http://127.0.0.1:3000/api/health" 