const {invoke} = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", ()=> {
  const boton = document.querySelector("#detectar-btn");
  const resultado = document.querySelector("#resultado");

  boton.addEventListener("click", async () => {
    resultado.textContent = "buscando....";

    const ruta = await invoke("detectar_stardew")

    if (ruta) {
      resultado.textContent = ruta
    } else {
      resultado.textContent = "NO HAY NADA "
    }
  })
})