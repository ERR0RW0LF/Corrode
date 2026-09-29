# Corrode
Rust based minecraft hosting solution

## Words

Instance = Minecraft server
Infrastructure = This project
vanilla = unmodded/default game

## Feature State

- [ ] Backend
- [ ] Frontend
- [ ] Multiple users
- [ ] Full isolation Instance based
- [ ] Creating vanilla Instances
- [ ] Creating modded Instances
- [ ] Adding mods using UI from Modrinth and CurseForge
- [ ] Creating modpack Instances
- [ ] Direct access to Instance files
- [ ] Settings of Instance
- [ ] Logs for Instances
- [ ] Logs for Infrastructure
- [ ] Commands
- [ ] Tab completion for commands
- [ ] Workflows (building, audit)
- [ ] Docker

## Tech-stack

Rust (Serving Frontend and Backend)
html/css/js (Client side)

Interacting with Instances will be done using custom lib written in rust.

Deploying using Docker.

### Creates

Axum, tokio, tower-http, serde (Api)

sqlx (PostgreSQL)

argon2 (Hash passwords)

tower-sessions, tower-sessions-sqlx-store, axum-login (Session handling)

bollard (Docker Api)

Supporting crates

| Need  |  Crate |
| --- | --- |
| HTTP client (Mojang, Fabric, Paper, Modrinth, CurseForge APIs ) |  reqwest with rustls-tls |
| Modpacks (.mrpack and CurseForge packs are zips ) |  zip |
| Verifying downloads  |  sha1 / sha2 |
| server.properties parsing  |  java-properties |
| Server status ping  |  craftping |
| Embedding html/css/js into the binary  |  rust-embed |
| Server-rendered HTML (optional ) |  askama or maud |
| Errors  |  thiserror in libs, anyhow in the binary |
| Logging  |  tracing, tracing-subscriber, tracing-appender |
| Config  |  figment (or config) plus clap for CLI flags |
| IDs and timestamps  |  uuid, time |
| Integration tests  |  testcontainers |
