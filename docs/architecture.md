# Arquitectura de KORDENT ERP

## Requisitos fundamentales

KORDENT ERP es una plataforma empresarial:

- Multiempresa.
- Global y neutral respecto a países concretos.
- Accesible mediante aplicación web.
- Disponible como aplicación de escritorio basada en la misma interfaz web.
- Capaz de funcionar sin conexión desde la aplicación de escritorio.
- Preparada para sincronizar los cambios locales cuando se recupere la conexión.

## Capas

- `kordent-core`: modelo, identificadores y reglas del dominio.
- `kordent-application`: casos de uso y puertos requeridos por la aplicación.
- Adaptadores de infraestructura: persistencia local, persistencia central y servicios externos.
- `kordent-api`: acceso remoto para la aplicación web y la sincronización.
- `kordent-worker`: tareas centrales en segundo plano.
- `kordent-cli`: administración por terminal.
- `kordent-web`: interfaz compartida para navegador y aplicación de escritorio.
- `apps/kordent-web/src-tauri`: host de escritorio Tauri que reutiliza la interfaz web y ejecuta lógica local en Rust.

El dominio no debe depender de bases de datos, frameworks web, sistemas operativos ni servicios externos.

## Multiempresa

`OrganizationId` identifica la empresa propietaria de los datos.

Todo dato empresarial deberá quedar asociado de forma explícita a una organización. Los casos de uso, consultas y repositorios deberán respetar ese ámbito y evitar accesos entre organizaciones.

Un usuario podrá recibir autorización para trabajar con una o varias organizaciones, pero los permisos y datos de cada una permanecerán aislados.

## Alcance global

El núcleo no incorporará reglas fiscales, monetarias o administrativas específicas de un país.

Los idiomas, monedas, zonas horarias, formatos y reglas regionales deberán ser configurables o implementarse mediante módulos separados.

## Aplicación web

La versión web utilizará la API central y requerirá conexión con el servidor.

La interfaz se implementará con React, TypeScript y Vite. El mismo frontend se reutilizará dentro de la aplicación de escritorio.

## Aplicación de escritorio

La aplicación de escritorio se construirá con Tauri y reutilizará la interfaz web.

Deberá poder instalarse, iniciarse y operar sin conexión. La lógica sensible y el acceso a datos locales permanecerán detrás de comandos y adaptadores Rust; el frontend no accederá directamente a las tablas.

## Persistencia local

La aplicación de escritorio utilizará SQLite como almacenamiento local.

Los cambios realizados sin conexión se confirmarán primero en la base local. Una cola de salida registrará las operaciones pendientes de sincronización.

Los UUID v7 permitirán crear identificadores localmente sin solicitar valores al servidor.

## Sincronización

Cuando se recupere la conexión, la aplicación de escritorio sincronizará automáticamente los cambios pendientes con la API central.

La sincronización deberá ser:

- Reintentable.
- Idempotente.
- Aislada por organización.
- Capaz de detectar conflictos.
- Segura frente a interrupciones parciales.

Las reglas concretas para resolver conflictos se definirán por agregado antes de implementar el motor de sincronización. No se permitirán sobrescrituras silenciosas.

## Decisiones pendientes

Todavía deben definirse:

- Motor de base de datos central.
- Protocolo de sincronización.
- Autenticación y autorización.
- Cifrado de los datos locales.
- Políticas de resolución de conflictos.
