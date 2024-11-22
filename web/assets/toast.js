let toast_len = 0;
function toast(type, msg) {
    const dialog = document.getElementById("dialog");
    const open = dialog && dialog.getAttribute("open") != null;
    
    // const toast_box = document.getElementsByClassName("toast-box")[index];
    const toast_box = document.createElement("div");
    toast_box.classList.add("toast-box");
    if (open) {
        dialog.appendChild(toast_box);
    } else {
        document.body.appendChild(toast_box);
    }

    const toast = document.createElement("div");
    toast.classList.add("toast");
    toast.classList.add(type);
    toast_box.appendChild(toast);
    const last_top = 10 + 40 * toast_len;
    toast.style.top = `${last_top - 40}px`
    toast_len += 1;

    setTimeout(() => {
        toast.style.top = `${last_top}px`
    }, 1);

    setTimeout(() => { // 自动消失
        toast.style.top = `${last_top - 40}px`
        toast.style.opacity = 0;
    }, 3 * 1000);

    setTimeout(() => { // 自动消失
        toast_box.removeChild(toast);
        toast_len -= 1;
    }, 3 * 1000 + 200);

    const msgSpan = document.createElement("span");
    msgSpan.innerHTML = msg;
    toast.appendChild(msgSpan);

    switch (type) {
        case "info":
            break;
        case "success":
            break;
        case "warn":
            break;
        case "error":
            break;
    }
}
