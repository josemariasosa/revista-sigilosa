# PostgreSQL Setup Guide

## Prerequisites

You need PostgreSQL installed and running on your machine.

### Option 1: Install PostgreSQL with Homebrew (macOS)

```bash
# Install PostgreSQL
brew install postgresql@16

# Start PostgreSQL service
brew services start postgresql@16

# Create the database
createdb sonido_sigiloso
```

### Option 2: Use Docker

```bash
# Run PostgreSQL in Docker
docker run -d \
  --name postgres-local \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=sonido_sigiloso \
  -p 5432:5432 \
  postgres:16

# The database will be available at:
# postgresql://postgres:postgres@localhost:5432/sonido_sigiloso
```

### Option 3: PostgreSQL.app (macOS)

Download and install from [https://postgresapp.com/](https://postgresapp.com/)

## Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your PostgreSQL connection details:
   ```
   DATABASE_URL=postgresql://username:password@localhost/sonido_sigiloso
   ```

3. Run the migrations (automatically done on startup):
   ```bash
   cargo run
   ```

## Database Connection

The application reads the database URL from the `DATABASE_URL` environment variable.

Default: `postgresql://postgres:postgres@localhost/sonido_sigiloso`

## Migrations

Migrations are automatically run on application startup using `sqlx::migrate!()`.

Migration files are located in `./migrations/`:
- `20260227000100_init.sql` - Initial tables
- `20260227000200_update_tracks.sql` - Add track fields
- `20260227000300_add_tone_constraint.sql` - Add tone validation
- `20260227000400_add_album_metadata.sql` - Add album metadata fields

## Verify Setup

```bash
# Check if PostgreSQL is running
psql -U postgres -l

# Connect to the database
psql -U postgres -d sonido_sigiloso

# Inside psql, list tables:
\dt
```

## Troubleshooting

### Connection refused
- Make sure PostgreSQL is running: `brew services list` or `docker ps`
- Check the port (default is 5432): `lsof -i :5432`

### Authentication failed
- Verify username/password in your `DATABASE_URL`
- For local PostgreSQL, you might need to edit `pg_hba.conf`

### Database does not exist
```bash
createdb sonido_sigiloso
# or
psql -U postgres -c "CREATE DATABASE sonido_sigiloso;"
```
