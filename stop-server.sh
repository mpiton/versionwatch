#!/bin/bash

echo "🛑 Stopping VersionWatch server..."

# Trouver et arrêter le processus versionwatch-cli
PID=$(ps aux | grep "versionwatch-cli" | grep -v grep | awk '{print $2}')

if [ -n "$PID" ]; then
    echo "📦 Stopping server (PID: $PID)"
    kill $PID
    sleep 2
    
    # Vérifier si le processus est toujours en cours
    if ps -p $PID > /dev/null; then
        echo "🔨 Force stopping server..."
        kill -9 $PID
    fi
    
    echo "✅ Server stopped successfully"
else
    echo "❌ No VersionWatch server found running"
fi 