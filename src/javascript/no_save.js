const { invoke } = window.__TAURI__.core;

const no_save_checkbox = document.getElementById("no_save_checkbox")

no_save_checkbox.addEventListener("change", async () => {
    const enabled = no_save_checkbox.checked;

    if(no_save_checkbox.checked) {
        console.log("enabled");
    } else {
        console.log("disabled");
    }

    try {
        await invoke("set_nosave_mode", { enabled: enabled });
    } catch (error) {
        console.error("rust fehler:", error);

        no_save_checkbox.checked = !enabled;
    }
});