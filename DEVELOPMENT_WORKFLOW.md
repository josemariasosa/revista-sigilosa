# Development Workflow

## 🚀 During Development (Now)

### Adding New Entregas to Init Data

Edit [init_data.json](init_data.json) and add new entregas and tracks:

```json
{
  "entregas": [
    {
      "name": "Primera Entrega",
      "batch_id": null,
      "created_at": "2026-02-26T00:00:00Z"
    },
    {
      "name": "Segunda Entrega",
      "batch_id": null,
      "created_at": "2026-02-27T00:00:00Z"
    }
  ],
  "tracks": [
    {
      "title": "5 Mouths",
      "artist_name": "Fred Fresh",
      "position": "A1",
      "score": "✅",
      "bpm": 126.46,
      "tone": "Db",
      "album_id": null,
      "duration_seconds": null,
      "entrega_id": 1,
      "created_at": "2026-02-26T00:00:00Z"
    },
    {
      "title": "New Song",
      "artist_name": "New Artist",
      "position": "A1",
      "score": "✅",
      "bpm": 128.0,
      "tone": "Am",
      "album_id": null,
      "duration_seconds": null,
      "entrega_id": 2,
      "created_at": "2026-02-27T00:00:00Z"
    }
  ]
}
```

**Note**: `entrega_id` in the JSON refers to the position in the entregas array (1 = first, 2 = second, etc.)

Run the app and **only the new entrega** will be added:

```bash
make run
```

Output:
```
🔧 Checking initial data...
  ⏭️  Entrega 'Primera Entrega' already exists, skipping
  ✅ Created entrega: Segunda Entrega
     → Added 1 tracks

✅ Initial data initialized successfully!
   → 1 entrega(s) created
   → 1 track(s) created
```

**✅ Benefits of using JSON:**
- No recompilation needed
- Easy to edit
- Can version control your initial data easily

## 🎯 Alpha/Production Phase (Later)

### Stop Using Init Data

Once you go to alpha:

1. **Don't add more to `init.rs`** - it's only for the initial "seed" entregas
2. **Use the API instead** to add new entregas/tracks:

```bash
# Add a new entrega
curl -X POST http://localhost:3000/entregas \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Tercera Entrega",
    "batch_id": null,
    "created_at": "2026-03-01T00:00:00Z"
  }'

# Add tracks
curl -X POST http://localhost:3000/tracks \
  -H "Content-Type: application/json" \
  -d '{
    "title": "New Song",
    "artist_name": "Artist Name",
    "position": "A1",
    "bpm": 128.0,
    "tone": "Am",
    "score": "✅",
    "entrega_id": 3,
    "created_at": "2026-03-01T00:00:00Z"
  }'
```

**✅ This data persists directly to `sonido_sigiloso.db`**

### Bulk Import (Alternative)

Or use the bulk import endpoint:

```bash
curl -X POST http://localhost:3000/import/json \
  -H "Content-Type: application/json" \
  -d @new_entrega.json
```

## 📊 Summary

| Phase | Add Data Via | Persists To DB? |
|-------|--------------|----------------|
| Development | `init.rs` or API | ✅ Yes |
| Alpha/Production | API only | ✅ Yes |

**Key Point**: Data added through ANY method (init.rs or API endpoints) persists permanently to the SQLite database file.

## 🔄 Reset During Development

Need a clean slate?

```bash
make resetDb  # Deletes database
make run      # Recreates with init.rs data
```
