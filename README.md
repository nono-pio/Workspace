# Workspace

## Backend

```lua
src-tauri/
|-- icons/
|-- target/
|
|-- src/
| |-- model/
| |--store/
| |--utils/
| |--error
| |--error
| |--main
|
|-- build.rs
|-- Cargo.lock
|-- Cargo.toml
|-- taur.conf.json
|-- ...
```

## Frontend

### Structure

```lua
src/
|-- app/
| |-- core/
| | |-- services/
| | |-- guards/
| | |-- interceptors/
| | |-- core.module.ts
| |
| |-- modules/
| | |-- users/
| | | |-- components/
| | | |-- services/
| | | |-- users-routing.module.ts
| | | |-- users.module.ts
| |
| |-- shared/
| | |-- components/
| | |-- directives/
| | |-- pipes/
| |
| |-- app-routing.module.ts
| |-- app.component.ts
| |-- app.module.ts
|
|-- assets/
| |-- images/
| |-- fonts/
|
|-- styles/
| |-- main.scss
|
|-- environments/
| |-- environment.ts
| |-- environment.prod.ts
|
|-- index.html
|-- main.ts
|-- ...
```
