import { listen } from '@tauri-apps/api/event';
import { getCurrent, WebviewWindow } from '@tauri-apps/api/window';
import { request } from './util/invoke';

let label = getCurrent().label;

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

function getQueryVariable(variable) {
    var query = window.location.search.substring(1);
    var vars = query.split("&");
    for (var i = 0; i < vars.length; i++) {
        var pair = vars[i].split("=");
        if (pair[0] == variable) { return pair[1]; }
    }
    return (false);
}





