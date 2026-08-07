const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  const vistaLista = document.querySelector("#vista-lista");
  const vistaDetalle = document.querySelector("#vista-detalle");
  const detalleContenido = document.querySelector("#detalle-contenido");

  // --- Detección del juego ---
  const detectarBtn = document.querySelector("#detectar-btn");
  const resultado = document.querySelector("#resultado");
  detectarBtn.addEventListener("click", async () => {
    resultado.textContent = "Buscando...";
    const ruta = await invoke("detectar_stardew");
    resultado.textContent = ruta
      ? `✅ Encontrado en: ${ruta}`
      : "❌ No encontré Stardew Valley en las rutas típicas.";
  });

  // --- Lista de mods populares ---
  const modsBtn = document.querySelector("#mods-btn");
  const listaMods = document.querySelector("#lista-mods");
  modsBtn.addEventListener("click", async () => {
    listaMods.textContent = "Cargando mods...";
    try {
      const mods = await invoke("mods_trending");
      listaMods.innerHTML = mods
        .map(
          (mod) => `
          <div class="mod" data-mod-id="${mod.mod_id}">
            ${mod.picture_url ? `<img src="${mod.picture_url}" alt="${mod.name}" />` : ""}
            <h3>${mod.name}</h3>
            <p>${mod.summary ?? "Sin descripción"}</p>
            <small>v${mod.version} · por ${mod.author}</small>
          </div>`
        )
        .join("");
    } catch (error) {
      listaMods.textContent = "Error: " + error;
    }
  });

  // --- Clic en una tarjeta → abrir su detalle ---
  listaMods.addEventListener("click", async (evento) => {
    const tarjeta = evento.target.closest(".mod");
    if (!tarjeta) return; // el clic no fue en una tarjeta

    const modId = Number(tarjeta.dataset.modId);
    await abrirDetalle(modId);
  });

  async function abrirDetalle(modId) {
  detalleContenido.textContent = "Cargando detalle...";
  vistaLista.style.display = "none";
  vistaDetalle.style.display = "block";
  try {
    const mod = await invoke("detalle_mod", { modId });
    const archivos = await invoke("archivos_mod", { modId });

    const archivosHtml = archivos
      .map(
        (a) => `
        <li class="archivo">
          <strong>${a.name}</strong> (v${a.version})<br />
          <small>${a.category_name ?? "—"} · ${a.size_kb ? Math.round(a.size_kb / 1024) + " MB" : "?"}</small>
        </li>`
      )
      .join("");

    detalleContenido.innerHTML = `
      ${mod.picture_url ? `<img src="${mod.picture_url}" alt="${mod.name}" />` : ""}
      <h2>${mod.name}</h2>
      <p>${mod.summary ?? "Sin descripción"}</p>
      <ul>
        <li>Versión: ${mod.version}</li>
        <li>Autor: ${mod.author}</li>
        <li>Subido por: ${mod.uploaded_by}</li>
        <li>👍 ${mod.endorsement_count ?? 0} endorsements</li>
        <li>⬇️ ${mod.mod_downloads ?? "?"} descargas</li>
      </ul>
      <h3>Archivos disponibles</h3>
      <ul class="archivos">${archivosHtml}</ul>`;
  } catch (error) {
    detalleContenido.textContent = "Error: " + error;
  }
}

  // --- Botón volver ---
  document.querySelector("#volver-btn").addEventListener("click", () => {
    vistaDetalle.style.display = "none";
    vistaLista.style.display = "block";
  });
});

