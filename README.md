# Sonido Sigiloso — Backend

Backend oficial de la revista musical **Sonido Sigiloso**.

Este sistema almacena, organiza y expone toda la información musical de la revista mediante una base de datos PostgreSQL, permitiendo que los artículos se rendericen con información en tiempo real.

---

## Quick Start

### Using Docker (Recommended)

```bash
# Start PostgreSQL database
make up

# Run the application (in another terminal)
make run
```

The app will automatically:
- Connect to PostgreSQL
- Run all migrations
- Load initial data from `init_data.json`
- Start the server at `http://localhost:3000`

### Using Local PostgreSQL (Homebrew)

If you already have PostgreSQL installed with Homebrew:

```bash
# Make sure PostgreSQL is running
brew services start postgresql@16

# Create the database (first time only)
createdb sonido_sigiloso

# Set environment variable (or use .env file)
export DATABASE_URL=postgresql://postgres:postgres@localhost/sonido_sigiloso

# Run the application
make run
```

### Stop

```bash
# Stop Docker PostgreSQL
make down
```

---

# Filosofía

Sonido Sigiloso es una revista viva.

No es un sitio estático. Es un sistema dinámico donde:

- La información musical vive en una base de datos estructurada (PostgreSQL)
- Los artículos consumen esa información en tiempo real
- La revista evoluciona con cada nuevo batch de vinilos
- El sistema puede correr completamente en local o en Docker
- La simplicidad es prioritaria
- La escalabilidad es posible sin rediseñar la arquitectura

---

# Conceptos principales

## Batch

Un batch representa un conjunto de discos adquiridos o analizados.

Ejemplo:

Batch 0008 — Fricciones Orgánicas

Un batch puede generar:

- nuevos albums
- nuevos tracks
- nuevas entregas de la revista

---

## Entrega

Una entrega es una publicación formal de la revista.

Ejemplo:

Entrega 0008 — Fricciones Orgánicas

Una entrega puede contener:

- artículos
- referencias a albums
- referencias a tracks
- referencias a artistas

---

## Artículo

Un artículo es contenido editorial.

El contenido se almacena como Markdown, pero puede consultar información en tiempo real desde la base de datos.

Ejemplo:

- Mostrar tracklist de un album
- Mostrar rating actualizado
- Mostrar metadata del artista

El artículo NO contiene la data hardcodeada.

La data vive en SQL.

---

## Programa en vivo

El sistema soportará registrar programas en vivo.

Ejemplo:

Programa en vivo — Jueves

Estos programas podrán referenciar:

- tracks reproducidos
- albums utilizados
- timestamps
- notas

---

# Objetivos técnicos

Este backend debe:

- correr como un único binario Rust
- usar SQLite como base de datos
- crear automáticamente la base de datos si no existe
- ejecutar migraciones automáticamente
- exponer un servidor HTTP local
- permitir consultas de la data
- permitir visualizar artículos
- permitir registrar batches
- permitir registrar entregas
- permitir registrar albums, artistas y tracks

---

# Arquitectura

Monolito local.

Componentes:

- SQLite database file
- Rust backend (Axum)
- SQLx para acceso a base de datos
- sistema de migraciones SQLx
- servidor HTTP local

No se usan contenedores.

No se usa infraestructura externa.

---

# Ubicación de la base de datos

La base de datos es un archivo local.

Ejemplo:

sonido_sigiloso.db


Puede ubicarse dentro del vault de Obsidian o dentro del repo.

---

# Modelo de datos inicial

Tablas principales:

- artists
- albums
- tracks
- batches
- entregas
- articles
- live_sets

---

# Funcionalidad inicial (MVP)

El sistema debe permitir:

- iniciar el servidor
- crear base de datos automáticamente
- crear tablas mediante migraciones
- consultar artists
- consultar albums
- consultar tracks
- consultar batches
- consultar entregas

Endpoints iniciales:

- GET /health
- GET /artists
- GET /albums
- GET /tracks
- GET /batches
- GET /entregas


---

# Stack tecnológico

Lenguaje:

Rust

Database:

SQLite

Framework web:

Axum

Database driver:

SQLx

Runtime:

Tokio

---

# Objetivo de diseño

Prioridades:

1. simplicidad
2. claridad
3. robustez
4. reproducibilidad
5. escalabilidad futura

---

# Uso esperado

Ejecutar:

cargo run

Luego abrir:

http://localhost:3000



---

# Futuro

Fases futuras incluirán:

- renderizado de artículos Markdown
- búsqueda full-text
- registro de live sets
- UI web
- importación automática desde Markdown
- exportación

---

# Estado

En desarrollo inicial.

