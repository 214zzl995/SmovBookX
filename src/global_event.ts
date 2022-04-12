import { listen } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { request } from './util/invoke';

let label = getCurrent().label;

if (label == "main") {
    !(async () => await request("listen_single"))()
}

!(async () => await listen(label + '_single', (event) => {
    let current = getCurrent()
    current.unminimize()
    current.setAlwaysOnTop(true)
    current.show()
    setTimeout(() => {
        current.setAlwaysOnTop(false)
    }, 50)
}))()

!(async () => await listen('tauri://file-drop-hover', event => {
    console.log(event)
})
)()

document.addEventListener('dragstart', (event) => {
    event.preventDefault();
}, false);





