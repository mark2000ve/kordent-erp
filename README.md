# KORDENT ERP

> El núcleo que coordina tu empresa.

KORDENT ERP es una plataforma empresarial modular desarrollada en Rust para conectar, controlar y coordinar las distintas áreas de una organización desde un núcleo común.

## Estado del proyecto

Proyecto en fase inicial de desarrollo.

## Estructura

- `apps/kordent-web`: interfaz web compartida con la futura aplicación de escritorio.
- `apps/kordent-web/src-tauri`: aplicación de escritorio Tauri que reutiliza la interfaz web.
- `apps/kordent-api`: API principal del sistema.
- `apps/kordent-worker`: procesamiento de tareas en segundo plano.
- `apps/kordent-cli`: herramientas de administración por terminal.
- `crates/kordent-application`: casos de uso y servicios de aplicación.
- `crates/kordent-core`: lógica y tipos compartidos del dominio.

## Arquitectura

La arquitectura multiempresa, global y con funcionamiento offline se describe en [`docs/architecture.md`](docs/architecture.md).

## Validación

### Rust

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```

### Interfaz web

```bash
npm ci --prefix apps/kordent-web
npm run lint --prefix apps/kordent-web
npm run build --prefix apps/kordent-web
```
