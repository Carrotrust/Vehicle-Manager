# vechicle-manager

Small Axum practice project for learning how routing and JSON handlers work in Rust.

## What it does

This app starts an HTTP server on `0.0.0.0:6570` and exposes one route:

- `GET /vehicle` returns a sample vehicle as JSON
- `POST /vehicle` accepts a vehicle JSON body and returns it with a generated `id`

The project is intentionally basic and uses in-memory request/response handling only. There is no database or persistent storage.

## Tech stack

- Rust
- Axum
- Tokio
- Serde
- UUID

## Project layout

```text
.
├── Cargo.toml
├── api-test.ps1
└── src
    ├── main.rs
    └── vehicle.rs
```

## Run the server

Make sure Rust is installed, then start the app:

```bash
cargo run
```

The server listens on:

```text
http://localhost:6570
```

## API

### GET `/vehicle`

Returns a sample vehicle object.

Example response:

```json
{
  "manufacturer": "toyota",
  "model": "camry-le",
  "year": 2018,
  "id": "generated-uuid"
}
```

### POST `/vehicle`

Send JSON without an `id` and the server will generate one.

Example request:

```json
{
  "manufacturer": "Tesla",
  "model": "CyberTruck",
  "year": 2025
}
```

Example response:

```json
{
  "manufacturer": "Tesla",
  "model": "CyberTruck",
  "year": 2025,
  "id": "generated-uuid"
}
```

## Quick test

With `curl`:

```bash
curl http://localhost:6570/vehicle
```

```bash
curl -X POST http://localhost:6570/vehicle \
  -H "Content-Type: application/json" \
  -d '{"manufacturer":"Tesla","model":"CyberTruck","year":2025}'
```

On Windows PowerShell, you can also use the included [`api-test.ps1`](/Users/mac/vechicle-manager/api-test.ps1) script as a reference.

## Notes

- `src/main.rs` wires up the Axum router and TCP listener.
- `src/vehicle.rs` contains the `Vehicle` struct and request handlers.
- The generated `id` uses a random UUID for each response.
