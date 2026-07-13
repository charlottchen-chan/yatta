const { invoke } = window.__TAURI__.core;

const no_save_checkbox = document.getElementById("no_save_checkbox")

no_save_checkbox.addEventListener("change", async () => {
    if (no_save_checkbox.checked) {
        await invoke('');
    }
});

