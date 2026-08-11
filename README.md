# 🌱 Stardew Mod Manager

Un gestor e instalador de mods de **Stardew Valley** para Windows, hecho con **Rust + Tauri**. Descubre, explora e instala mods de [Nexus Mods](https://www.nexusmods.com/stardewvalley) desde una app de escritorio ligera — sin andar moviendo carpetas a mano.

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-24C8B8?logo=tauri&logoColor=white)
![Estado](https://img.shields.io/badge/estado-en%20desarrollo-yellow)



---

## ✨ Características

| | Función |
|---|---|
| ✅ | Detección automática de la instalación de Stardew (Steam en **cualquier disco** vía `libraryfolders.vdf`, y GOG) |
| ✅ | Explorar los mods **populares** de Stardew desde Nexus Mods |
| ✅ | Página de **detalle** de cada mod: imágenes, versión, autor, endorsements y descargas |
| ✅ | Lista de **archivos descargables** por mod |
| ✅ | Recibe enlaces `nxm://` (estilo Vortex) y resuelve el link de descarga real desde Nexus |
| 🚧 | Descarga del archivo del mod desde el CDN — *en progreso* |
| ⏳ | Instalación automática en la carpeta `Mods` de SMAPI (con backup) |
| ⏳ | Resolución de dependencias entre mods |
| ⏳ | Detección de conflictos |

---

## 🛠️ Tecnologías

- **Rust** — toda la lógica pesada: sistema de archivos, llamadas a la API, descargas.
- **Tauri v2** — app de escritorio con binario ligero (usa el WebView2 nativo de Windows, no empaqueta un navegador).
- **Frontend Vanilla** — HTML / CSS / JavaScript, sin frameworks.
- **Nexus Mods API** — consumida con `reqwest` + `serde`.

---

## 📋 Requisitos

**Para usar la app:**
- Windows 10/11
- Stardew Valley instalado (Steam o GOG)
- [SMAPI](https://smapi.io/) — el loader de mods de Stardew
- Una cuenta de [Nexus Mods](https://www.nexusmods.com/) + tu **API key** personal

**Para desarrollar:**
- [Rust](https://rustup.rs/) (rustup)
- [Node.js](https://nodejs.org/) (LTS)
- **Visual Studio Build Tools** con el workload *"Desktop development with C++"*
- **WebView2** (ya viene con Windows 11)

---

## 🚀 Puesta en marcha (desarrollo)

1. **Clona el repo** y entra a la carpeta.

2. **Instala las dependencias del frontend:**
   ```bash
   npm install
   ```

3. **Consigue tu API key de Nexus:** entra a *Account Settings → API Keys* en Nexus y genera tu *Personal API Key*.

4. **Crea el archivo `src-tauri/.env`** con tu key:
   ```
   NEXUS_API_KEY=tu_api_key_aqui
   ```
   > Este archivo está en `.gitignore` — **nunca** se sube al repo. Trata tu key como una contraseña.

5. **Corre en modo desarrollo:**
   ```bash
   npm run tauri dev
   ```
   La primera compilación tarda varios minutos (Rust compila todas las dependencias). Las siguientes son casi instantáneas.

---

## 📁 Estructura del proyecto

```
stardew-mod-manager/
├── src/                  # Frontend (lo que se ve)
│   ├── index.html
│   ├── main.js           # llama a los comandos de Rust
│   └── styles.css
└── src-tauri/            # Backend en Rust
    ├── src/lib.rs        # comandos: detección, API de Nexus, deep-link...
    ├── Cargo.toml        # dependencias de Rust
    ├── tauri.conf.json   # configuración de la app
    └── .env              # tu API key (ignorado por git)
```

> **Dos mundos:** el frontend (web) solo pinta la interfaz; toda la lógica que toca disco/red vive en Rust. Se comunican por *comandos* de Tauri (`invoke`).

---

## 🗺️ Roadmap

- [x] **Fase 0** — Setup + detección del juego
- [x] **Fase 1** — Cliente de Nexus + navegador de mods (lista, detalle, archivos)
- [ ] **Fase 2** — Descarga (`nxm://`) e instalación con backup
- [ ] **Fase 3** — Resolución de dependencias entre mods
- [ ] **Fase 4** — Detección de conflictos
- [ ] **Fase 5** — Pulido de la interfaz

---

## 📝 Notas

- Usa la API pública de Nexus Mods; respeta sus límites de uso.
- Proyecto personal / de aprendizaje. No está afiliado con ConcernedApe ni con Nexus Mods.
