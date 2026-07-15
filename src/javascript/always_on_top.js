const { invoke } = window.__TAURI__.core;

const always_on_top_checkbox = document.getElementById('always_on_top_checkbox')


always_on_top_checkbox.addEventListener('change', async () => {
    const isChecked = always_on_top_checkbox.checked;
    await invoke('alwaysontop', { isAlwaysOnTop: isChecked });
    await invoke('update_settings', {
        key: 'always_on_top',
        value: isChecked ? 'true' : 'false'
    });
});

