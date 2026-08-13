// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use url::{Url, ParseError};
use std::path::{Path, PathBuf};
use tauri_plugin_deep_link::DeepLinkExt;



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct UsuarioNexus {
    name: String,
    is_premium: bool,
    user_id: u64,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ModNexus{
    mod_id: u64,
    name: String,
    summary: Option<String>,
    version: String,
    author: String,
    picture_url: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ModDetalle {
    mod_id: u64,
    name: String,
    summary: Option<String>,
    picture_url: Option<String>,
    version: String,
    author: String,
    uploaded_by: String,
    endorsement_count: Option<u64>,
    mod_downloads: Option<u64>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RespuestaArchivos {
    files: Vec<ArchivoMod>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ArchivoMod {
    file_id: u64,
    name: String,
    version: String,
    category_name: Option<String>,
    size_kb: Option<u64>,
    file_name: String,
}

#[derive(serde::Deserialize)]
struct Mirror {
    #[serde(rename = "URI")]
    uri: String,
}


#[derive(serde::Deserialize, serde::Serialize)]
struct UrlParseada{
    game_domain: String,
    mod_id: u64,
    file_id:u64,
    key: String,
    expires: String,
}

#[tauri::command]
async fn archivos_mod(mod_id: u64) -> Result<Vec<ArchivoMod>, String> {
    let key = std::env::var("NEXUS_API_KEY")
        .map_err(|_| "No hay API key configurada".to_string())?;

    let url = format!("https://api.nexusmods.com/v1/games/stardewvalley/mods/{mod_id}/files.json");

    let respuesta: RespuestaArchivos = reqwest::Client::new()
        .get(&url)
        .header("apikey", key)
        .header("User-Agent", "StardewModManager/0.1")
        .send()
        .await
        .map_err(|e| format!("Error de red: {e}"))?
        .json()
        .await
        .map_err(|e| format!("No pude entender la respuesta: {e}"))?;

    Ok(respuesta.files)   // ← devolvemos SOLO el vector de adentro del sobre
}

#[tauri::command]
async fn detalle_mod(mod_id: u64) -> Result<ModDetalle, String>{
    let key = std::env::var("NEXUS_API_KEY")
        .map_err(|_| "No hay API key configurada".to_string())?;

    let url = format!("https://api.nexusmods.com/v1/games/stardewvalley/mods/{mod_id}.json");

    let mod_detalle: ModDetalle = reqwest::Client::new()
        .get(&url)
        .header("apikey", key)
        .header("User-Agent", "StardewModManager/0.1")
        .send()
        .await
        .map_err(|e| format!("Error de red: {e}"))?
        .json()
        .await
        .map_err(|e| format!("No pude entender la respuesta: {e}"))?;

    Ok(mod_detalle)
}

#[tauri::command]
async fn mods_trending() -> Result<Vec<ModNexus>, String>{
    let key = std::env::var("NEXUS_API_KEY")
        .map_err(|_| "No hay API key configurada".to_string())?;

    let mods: Vec<ModNexus> = reqwest::Client::new()
        .get("http://api.nexusmods.com/v1/games/stardewvalley/mods/trending.json")
        .header("apikey", key)
        .header("User-Agent", "StardewModManager/0.1")
        .send()
        .await
        .map_err(|e| format!("Error de red: {e}"))?
        .json()
        .await
        .map_err(|e |format!("No pude entender la respuesta: {e}"))?;

    Ok(mods)
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

#[tauri::command]
async fn validar_nexus() -> Result<UsuarioNexus , String>{

    let key = std::env::var("NEXUS_API_KEY")
        .map_err(|_| "No hay API key configurada".to_string())?;

    let respuesta = reqwest::Client::new()
        .get("https://api.nexusmods.com/v1/users/validate.json")
        .header("apikey",key)
        .header("User-Agent", "StardewModManager/0.1")
        .send()
        .await
        .map_err(|e| format!("Error de red: {e}"))?;

    let usuario: UsuarioNexus = respuesta
        .json()
        .await
        .map_err(|e| format!("No pude leer la respuesta: {e}"))?;


    Ok(usuario)

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
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| { /* ... */ }))
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {

            app.deep_link().register_all()?;
            
            let start_urls = app.deep_link().get_current()?;
            if let Some(urls) = start_urls {
            tauri::async_runtime::spawn(async move {
                match manejar_urls(urls).await {
            Ok(link) => println!("Link de descarga: {}", link),
            Err(e)   => println!("Error: {}", e),
        }
    });
}

            app.deep_link().on_open_url(|event|{
                tauri::async_runtime::spawn(async move {
                match manejar_urls(event.urls()).await {
            Ok(link) => println!("Link de descarga: {}", link),
            Err(e)   => println!("Error: {}", e)
             }
    });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![archivos_mod,suma,detectar_stardew,nexus_key_cargada,validar_nexus,mods_trending,detalle_mod])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


async fn manejar_urls(urls: Vec<Url>) -> Result<String,String>{
    println!("deep links urls: {:?}", urls);
     let keyapi = std::env::var("NEXUS_API_KEY")
        .map_err(|_| "No hay API key configurada".to_string())?;
    let mut key = String::new();       // arranca vacia
    let mut expires = String::new();

    for url in urls {
            let partes: Vec<&str> = url.path().split('/').collect();
            println!("{:?}", partes);  

            let mod_id = partes[2];
            let file_id = partes[4]; 

            for (clave, valor ) in url.query_pairs(){
                if clave == "key"{
                    key = valor.to_string();
            }else if clave == "expires"{
                    expires = valor.to_string();
            }

            


            
      
    }
    println!("key = {}, expires = {}, mod_id = {}, file_id = {}", key, expires,mod_id,file_id);
    
    let urlFinal = format!("https://api.nexusmods.com/v1/games/stardewvalley/mods/{mod_id}/files/{file_id}/download_link.json?key={key}&expires={expires}");


   


    let respuesta = reqwest::Client::new()
        .get(urlFinal)
        .header("apikey",keyapi)
        .header("User-Agent", "StardewModManager/0.1")
        .send()
        .await
        .map_err(|e| format!("Error de red: {e}"))?;

    let mirrors: Vec<Mirror> = respuesta
        .json()
        .await
        .map_err(|e| format!("No se puedo leer la respuesta: {e}"))?;

    let link = mirrors[0].uri.clone();

    let ruta = descargar_archivo(&link).await?;

    

      
 
    println!("{}",link);

    return Ok(link)
    
}

Err("no llego ninguna url".to_string())
}



async fn descargar_archivo(link: &str) -> Result<PathBuf, String> {

    let file = reqwest::get(link)
        .await.map_err(|e|format!("Hubo un error {e}"))?
        .bytes()
        .await.map_err(|e|format!("Hubo un error {e}"))?;
    // 1. GET al link  ->  saca los bytes


    // 2. arma la ruta destino: temp_dir() + join(un nombre .zip)

    let carpeta = std::env::temp_dir().join("mod.zip");
    // 3. escribe los bytes en esa ruta  (std::fs::write)
    std::fs::write(&carpeta, &file)
        .map_err(|e| format!("No se pudo escribir el archivo {e}"))?;

    // 4. devuelve la ruta:  Ok(ruta)

    Ok(carpeta)
}
