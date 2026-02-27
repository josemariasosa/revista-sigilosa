#!/bin/bash

# Script to import additional data into the application

echo "📥 Importing data..."

curl -X POST http://localhost:3000/import/json \
  -H "Content-Type: application/json" \
  -d @init_data.json

echo ""
echo "✅ Data import completed!"
echo ""
echo "View your tracks at: http://localhost:3000/tracks"
echo "View entregas at: http://localhost:3000/entregas"
