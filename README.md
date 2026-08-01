# KORDENT ERP

> El núcleo que coordina tu empresa.

KORDENT ERP es una plataforma empresarial modular desarrollada en Rust para conectar, controlar y coordinar las distintas áreas de una organización desde un núcleo común.

## Estado del proyecto

Proyecto en fase inicial de desarrollo.

## Estructura

- `apps/kordent-api`: API principal del sistema.
- `apps/kordent-worker`: procesamiento de tareas en segundo plano.
- `apps/kordent-cli`: herramientas de administración por terminal.
- `crates/kordent-application`: casos de uso y servicios de aplicación.
- `crates/kordent-core`: lógica y tipos compartidos del dominio.

## Validación

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
