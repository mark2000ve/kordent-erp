# KORDENT Web

Interfaz web de KORDENT ERP, construida con React, TypeScript y Vite.

Esta interfaz se utilizará desde el navegador y se reutilizará en la futura aplicación de escritorio basada en Tauri.

## Requisitos

- Node.js 24
- npm 11

Desde la raíz del repositorio, `nvm use` activa la versión indicada en `.nvmrc`.

## Desarrollo

```bash
npm install --prefix apps/kordent-web
npm run dev --prefix apps/kordent-web
```

## Validación

```bash
npm run lint --prefix apps/kordent-web
npm run build --prefix apps/kordent-web
```

La aplicación web utilizará la API central. El funcionamiento completamente offline se proporcionará mediante la aplicación de escritorio y su almacenamiento local.

## Aplicación de escritorio

La aplicación de escritorio utiliza Tauri y reutiliza este mismo frontend.

```bash
npm run desktop:dev --prefix apps/kordent-web
```

Tauri requiere las dependencias nativas correspondientes al sistema operativo donde se compile. El desarrollo Linux puede ejecutarse mediante WSLg.
