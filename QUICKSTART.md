# 🚀 Quick Start

## Primera vez

```bash
make run
```

¡Eso es todo! La aplicación automáticamente:
- ✅ Crea la base de datos
- ✅ Ejecuta las migraciones  
- ✅ Inicializa con datos iniciales (8 tracks de la "Primera Entrega")
- ✅ Inicia el servidor en http://localhost:3000

## Ver los datos

- Tracks: http://localhost:3000/tracks
- Entregas: http://localhost:3000/entregas
- Admin: http://localhost:3000/admin

## Comandos útiles

```bash
make help            # Ver todos los comandos
make showMigrations  # Ver migraciones disponibles (estilo Django)
make migrate         # Info sobre cómo aplicar migraciones
make resetDb         # Resetear base de datos
make importData      # Importar datos adicionales (con app corriendo)
make build           # Compilar para producción
make test            # Ejecutar tests
```

## ¿Cómo funciona la inicialización automática?

El sistema usa un patrón **fail-fast e idempotente** (inspirado en NEAR blockchain):

1. Al iniciar, verifica si existen entregas en la BD
2. Si está vacía → Inserta los datos iniciales
3. Si ya tiene datos → Se salta (no duplica)

Puedes reiniciar la app sin preocuparte de duplicar datos. Los datos iniciales solo se insertan la primera vez.

## Gestión de Migraciones (Estilo Django)

Ver todas las migraciones:
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

Las migraciones se aplican automáticamente al ejecutar `make run`.

## Documentación completa

Ver [INIT_DATA_GUIDE.md](INIT_DATA_GUIDE.md) para más detalles.
