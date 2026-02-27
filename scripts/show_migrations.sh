#!/bin/bash

# Show all migrations in the migrations directory

echo "📋 Available Migrations:"
echo ""

if [ ! -d "migrations" ]; then
    echo "❌ No migrations directory found"
    exit 1
fi

migrations=$(ls -1 migrations/*.sql 2>/dev/null | sort)

if [ -z "$migrations" ]; then
    echo "❌ No migration files found"
    exit 0
fi

count=0
for migration in $migrations; do
    count=$((count + 1))
    filename=$(basename "$migration")
    echo "  [$count] $filename"
    
    # Show first comment line if available
    first_comment=$(head -n 5 "$migration" | grep -E "^--" | head -n 1 | sed 's/^-- //')
    if [ ! -z "$first_comment" ]; then
        echo "      → $first_comment"
    fi
    echo ""
done

echo "Total: $count migration(s)"
