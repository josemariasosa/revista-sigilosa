# Cómo Inicializar Datos en tu Aplicación

## ✅ Sistema de Init Data Automático (Fail Fast & First)

- ✅ Se ejecuta automáticamente al iniciar la aplicación
- ✅ **Solo corre si la base de datos está vacía** (fail fast if initialized)
- ✅ Inserta los datos iniciales una sola vez
- ✅ Si ya existen datos, se salta la importación
- ✅ **No requiere pasos manuales** para inicializar

### Cómo Funciona

Cuando ejecutas `cargo run` o `make run`, la aplicación:

1. Crea la base de datos (si no existe)
2. Ejecuta las migraciones
3. **Verifica si hay entregas en la base de datos**
4. Si está vacía → Inserta los datos iniciales automáticamente
5. Si ya tiene datos → Continúa sin hacer nada

```
🔧 Initializing database with initial data...
✅ Initial data created successfully!
   → 1 entrega created
   → 8 tracks created
```

O si ya está inicializada:

```
⏭️  Database already initialized, skipping init data
```

## ✅ Cambios Realizados

He actualizado tu aplicación para soportar tus datos de entregas musicales:

### 1. **Nueva Migración de Base de Datos**
   - Archivo: `migrations/20260227000200_update_tracks.sql`
   - Cambios:
     - ✅ `bpm`: Ahora soporta decimales (126.46, 129.25, etc.)
     - ✅ `tone`: Acepta claves complejas como "Db", "Fm", "Bb", "Cm"
     - ✅ `position`: Nuevo campo para A1, A2, B1, B2, etc.
     - ✅ `score`: Nuevo campo para emojis de calificación (✅, 🟡, 🔴, 😈)
     - ✅ `artist_name`: Nombre del artista directamente en el track
     - ✅ `entrega_id`: Vincula tracks con entregas/releases

### 2. **Modelos Actualizados**
   - `src/models.rs` actualizado con los nuevos campos

### 3. **Rutas Actualizadas**
   - Todos los endpoints de tracks ahora soportan los nuevos campos
   - El endpoint `/import/json` está listo para importar tus datos

### 4. **Archivo de Datos Iniciales**
   - `init_data.json` contiene tus 8 tracks listos para importar

## 🚀 Cómo Usar

### Inicio Rápido (Recomendado)

**Solo necesitas hacer esto:**

```bash
make run
```

¡Y eso es todo! La aplicación automáticamente:
- Crea la base de datos
- Ejecuta las migraciones
- Inserta los datos iniciales (si la BD está vacía)
- Inicia el servidor

### Ver Migraciones Disponibles

```bash
make showMigrations
```

Esto muestra todas las migraciones en el directorio `migrations/`.

### Aplicar Migraciones

```bash
make migrate
# o simplemente
make run
```

Las migraciones se aplican automáticamente al iniciar la app.

### Resetear la Base de Datos

Si quieres empezar desde cero:

```bash
make resetDb
make run
```

### Método Alternativo: Importación Manual

Si prefieres importar datos manualmente después de iniciar la app:

**Paso 1: Eliminar la base de datos anterior (si existe):**
```bash
rm sonido_sigiloso.db
```

**Paso 2: Iniciar la aplicación:**
```bash
make run
# o
cargo run
```

**Paso 3: Importar datos adicionales (en otra terminal):**
```bash
make importData
```

## 🐞 Gestión de Migraciones (Estilo Django)

### Ver todas las migraciones

```bash
make showMigrations
```

Output:
```
📋 Available Migrations:

  [1] 20260227000100_init.sql

  [2] 20260227000200_update_tracks.sql
      → Add new fields to tracks table and fix data types

Total: 2 migration(s)
```

### Aplicar migraciones

Las migraciones se aplican automáticamente al iniciar:

```bash
make run
```

O si prefieres ser explícito:

```bash
make migrate  # Muestra cómo aplicar migraciones
```

## 📝 Agregar Más Entregas

Para agregar más entregas en el futuro, puedes:

### 1. Crear un nuevo archivo JSON:
```json
{
  "entregas": [
    {
      "name": "Segunda Entrega",
      "batch_id": null,
      "created_at": "2026-03-01T00:00:00Z"
    }
  ],
  "tracks": [
    {
      "title": "Nombre de Canción",
      "artist_name": "Nombre del Artista",
      "position": "A1",
      "score": "✅",
      "bpm": 128.0,
      "tone": "Am",
      "entrega_id": 2,
      "created_at": "2026-03-01T00:00:00Z"
    }
  ]
}
```

### 2. Importarlo:
```bash
curl -X POST http://localhost:3000/import/json \
  -H "Content-Type: application/json" \
  -d @tu_archivo.json
```

O usa el comando make:
```bash
make importData
```

## 🎯 Mejores Prácticas

### 1. Inicio Limpio
- **Primera vez**: Solo ejecuta `make run` - los datos iniciales se cargan automáticamente
- **Reset completo**: Usa `make resetDb` seguido de `make run`

### 2. Agregar Más Datos
- **Opción A (Código)**: Modifica [src/init.rs](src/init.rs) y agrega más datos al array
- **Opción B (JSON)**: Usa el endpoint `/import/json` o `make importData` con nuevos datos en [init_data.json](init_data.json)
- **Opción C (API)**: Usa los endpoints REST individuales

### 3. Desarrollo
- **Migraciones**: Para cambios de schema, siempre crea una nueva migración en `migrations/`
- **Init Data**: Mantén los datos iniciales en `src/init.rs` para que siempre estén disponibles
- **JSON Import**: Usa `/import/json` para cargas masivas adicionales

### 4. Ver Migraciones
- Usa `make showMigrations` para ver todas las migraciones disponibles
- Las migraciones se aplican automáticamente con `make run`

## 📋 Comandos Makefile Disponibles

```bash
make help            # Ver todos los comandos disponibles
make run             # Ejecutar la aplicación
make build           # Compilar en modo release
make test            # Ejecutar tests
make clean           # Limpiar artifacts y base de datos
make migrate         # Info sobre migraciones
make showMigrations  # Mostrar todas las migraciones
make importData      # Importar init_data.json (requiere app corriendo)
make resetDb         # Resetear la base de datos
```

## 🔍 Endpoints Disponibles

- `GET /tracks` - Ver todos los tracks
- `POST /tracks` - Crear un track
- `PUT /tracks/{id}` - Actualizar un track
- `GET /entregas` - Ver todas las entregas
- `POST /entregas` - Crear una entrega
- `POST /import/json` - Importar datos en masa

## 💡 Notas Importantes

- ✅ **Fail Fast**: Si la app detecta datos existentes, NO los sobrescribe
- ✅ **Idempotente**: Puedes reiniciar la app sin preocuparte - no duplicará datos
- ✅ **Automático**: No necesitas pasos manuales para inicializar
- ✅ **Estilo NEAR**: Patrón de inicialización inspirado en NEAR blockchain
- ✅ **Migraciones Django-style**: Comandos `make migrate` y `make showMigrations`
- ✅ Los emojis en el campo `score` están soportados (✅, 🟡, 🔴, 😈)
- ✅ El BPM ahora acepta decimales para mayor precisión
- ✅ **Tonos musicales validados**: Se permiten todos los tonos estándar:
  - **Mayores**: C, C#/Db, D, D#/Eb, E, E#/Fb, F, F#/Gb, G, G#/Ab, A, A#/Bb, B, B#/Cb
  - **Menores**: Cm, C#m/Dbm, Dm, D#m/Ebm, Em, E#m/Fbm, Fm, F#m/Gbm, Gm, G#m/Abm, Am, A#m/Bbm, Bm, B#m/Cbm
- ✅ El campo `position` te ayuda a ordenar tracks por su ubicación física en un disco o tracklist

## 🔧 Archivos Clave

- [src/init.rs](src/init.rs) - Lógica de inicialización automática
- [init_data.json](init_data.json) - Datos de ejemplo para importación manual
- [scripts/import_seed.sh](scripts/import_seed.sh) - Script de importación manual
- [scripts/show_migrations.sh](scripts/show_migrations.sh) - Script para mostrar migraciones
- [Makefile](Makefile) - Comandos útiles para desarrollo
- [migrations/](migrations/) - Directorio de migraciones SQL
