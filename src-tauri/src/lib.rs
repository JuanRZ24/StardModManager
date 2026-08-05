// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{fs::read_to_string, path::{Path, PathBuf}};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn suma(a:i32, b:i32) -> i32 {
    a + b
}

#[tauri::command]
fn detectar_stardew() -> Option<String> {
   let mut candidatos: Vec<PathBuf> = Vec::new();

   if let Some(steam_base) = encontrar_steam_base(){
    for biblioteca in leer_bibliotecas_steam(&steam_base){
        candidatos.push(
            biblioteca
                .join("steamapps")
                .join("common")
                .join("Stardew Valley"),
        );
    }
   }

    candidatos.push(PathBuf::from(r"C:\GOG Games\Stardew Valley"));
    candidatos.push(PathBuf::from(r"C:\Program Files (x86)\GOG Galaxy\Games\Stardew Valley"));

    for carpeta in candidatos{
        if es_carpeta_stardew(&carpeta){
            return Some(carpeta.to_string_lossy().to_string());
        }
    }

    None

}

fn es_carpeta_stardew(carpeta: &Path) -> bool{
    carpeta.join("Stardew Valley.exe").exists()
      || carpeta.join("StardewValley.exe").exists()
}

fn encontrar_steam_base()-> Option<PathBuf> {
    let posibles = [
        r"C:\Program Files (x86)\Steam",
        r"C:\Program Files\Steam",
    ];

    for ruta in posibles{
        let carpeta = PathBuf::from(ruta);
        if carpeta.join("steam.exe").exists(){
            return Some(carpeta)
        }
    }
    None
}


fn leer_bibliotecas_steam(steam_base: &Path) -> Vec<PathBuf> {
    let mut bibliotecas: Vec<PathBuf> = Vec::new();

    let archivo_vdf = steam_base.join("steamapps").join("libraryfolders.vdf");

    let contenido = match std::fs::read_to_string(&archivo_vdf){
        Ok(texto) => texto,
        Err(_) => return bibliotecas,
    };

    for linea in contenido.lines(){
        let linea = linea.trim();
        if linea.starts_with("\"path\""){
            let partes: Vec<&str> = linea.split('"').collect();
            if partes.len() >= 4{
                let ruta = partes[3].replace("\\\\", "\\");
                bibliotecas.push(PathBuf::from(ruta));
            }
        }
    }
    bibliotecas
}

#[tauri::command]
fn nexus_key_cargada() -> bool {
    std::env::var("NEXUS_API_KEY").is_ok()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,suma,detectar_stardew,nexus_key_cargada])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
