function copyUrl(url) {
    const protocol = window.location.protocol; // http:
    const host = window.location.host; // localhost:8080
    url = url.replace("./", "");
    let text = `${protocol}//${host}/${url}`;
    console.log(text);
    let ok = copyText(text)
    // console.log("ok?", ok)
    if (ok) {
        toast("success", "已复制~")
    } else {
        toast("error", "复制失败，请检查是否授权操作剪贴板~")
    }
}

/**
     * 复制指定文本到剪贴板
     * @param text
     * @returns {boolean}
     */
function copyText(text) {
    if (text == null || text === '') {
        return false;
    }
    let textArea = document.createElement("textarea");
    textArea.value = text;
    textArea.style.position = "fixed";

    const dialog = document.getElementById("dialog");
    const open = dialog && dialog.getAttribute("open") != null;
    let ele = null;
    if (open) {
        ele = dialog;
    } else {
        ele = document.body;
    }
    if (!ele) {
        return
    }
    ele.appendChild(textArea);
    textArea.select();
    try {
        let ok = document.execCommand("copy");
        // console.log("copy==", ok)
        return ok;
    } catch (err) {
        console.log('复制文本到剪贴板失败', err);
        return false;
    } finally {
        ele.removeChild(textArea);
    }
}

function setCookie(cname, cvalue, exdays=30) {
    let d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    let expires = "expires=" + d.toGMTString();
    document.cookie = cname + "=" + cvalue + "; " + expires;
}

function getCookie(cname) {
    let name = cname + "=";
    let ca = document.cookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i].trim();
        if (c.indexOf(name) == 0) return c.substring(name.length, c.length);
    }
    return "";
}

const theme_cookie_key = "easyImageDarkTheme";
function initThemeBtn() {
    const themeEle = document.createElement("div");
    const btn = document.createElement("div");
    btn.addEventListener('click', () => changeTheme(btn));
    themeEle.classList.add("theme");
    themeEle.appendChild(btn);
    document.body.appendChild(themeEle);
    changeTheme(btn, true);
}

function changeTheme(btn, init) {
    let darkValue = getCookie(theme_cookie_key);
    let dark = darkValue == "" ? false : true;
    
    if (init != true) {
        // console.log("change", darkValue, dark);
        dark = !dark;
    }
    setCookie(theme_cookie_key, dark ? "1" : "");
    btn.innerHTML = dark ? "明亮" : "黑暗";

    const root=document.querySelector(":root")
    if (dark) {
        root.style.setProperty("--text","white")
        root.style.setProperty("--bg","black")
        root.style.setProperty("--primary","rgb(108 173 151)")
        root.style.setProperty("--shadow","rgba(255, 255, 255, 0.15)")
    } else {
        root.style.setProperty("--bg","white")
        root.style.setProperty("--text","black")
        root.style.setProperty("--primary","rgb(108 173 151)")
        root.style.setProperty("--shadow","rgba(0, 0, 0, 0.1)")
    }
}