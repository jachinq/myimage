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
    let exist = document.getElementsByClassName("theme");
    if (exist.length > 0) {
        return;
    }
    const themeEle = document.createElement("div");
    const btn = document.createElement("a");
    btn.addEventListener('click', () => changeTheme(btn));
    themeEle.classList.add("theme");
    themeEle.appendChild(btn);
    // document.body.appendChild(themeEle);
    document.getElementsByClassName("header")[0].appendChild(themeEle);
    changeTheme(btn, true);
}
initThemeBtn();

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
        // 旧主题变量
        root.style.setProperty("--text","var(--light)")
        root.style.setProperty("--text-hover","var(--dark)")
        root.style.setProperty("--bg","var(--dark)")
        root.style.setProperty("--bg-header","var(--header-dark)")
        root.style.setProperty("--primary","var(--primary-dark)")
        root.style.setProperty("--primary-hover","var(--primary-light)")
        root.style.setProperty("--shadow","0px 0px 0px ragb(0,0,0,0)")
        // 新设计系统变量
        root.style.setProperty("--bg-primary", "#2c2c33ff")
        root.style.setProperty("--bg-secondary", "#16161d")
        root.style.setProperty("--bg-card", "#39393eff")
        root.style.setProperty("--bg-glass", "rgba(255, 255, 255, 0.03)")
        root.style.setProperty("--bg-glass-hover", "rgba(255, 255, 255, 0.06)")
        root.style.setProperty("--text-primary", "#e4e4e7")
        root.style.setProperty("--text-secondary", "#a1a1aa")
        root.style.setProperty("--text-muted", "#71717a")
        root.style.setProperty("--border", "rgba(255, 255, 255, 0.08)")
        root.style.setProperty("--border-hover", "rgba(255, 255, 255, 0.15)")
    } else {
        // 旧主题变量
        root.style.setProperty("--text","var(--dark)")
        root.style.setProperty("--text-hover","var(--light)")
        root.style.setProperty("--bg","var(--light)")
        root.style.setProperty("--bg-header","var(--header-light)")
        root.style.setProperty("--primary","var(--primary-light)")
        root.style.setProperty("--primary-hover","var(--primary-dark)")
        root.style.setProperty("--shadow","0 2px 4px rgba(0, 0, 0, 0.1)")
        // 新设计系统变量
        root.style.setProperty("--bg-primary", "#f8f8f2")
        root.style.setProperty("--bg-secondary", "#ffffff")
        root.style.setProperty("--bg-card", "#ffffff")
        root.style.setProperty("--bg-glass", "rgba(0, 0, 0, 0.02)")
        root.style.setProperty("--bg-glass-hover", "rgba(0, 0, 0, 0.05)")
        root.style.setProperty("--text-primary", "#39393eff")
        root.style.setProperty("--text-secondary", "#52525b")
        root.style.setProperty("--text-muted", "#a1a1aa")
        root.style.setProperty("--border", "rgba(0, 0, 0, 0.08)")
        root.style.setProperty("--border-hover", "rgba(0, 0, 0, 0.15)")
    }
}

// const resetContainerMarginTop = () => {
//     let header = document.getElementsByClassName("header")[0];
//     let container = document.getElementsByClassName("container")[0];
//     if (!header || !container) {
//         return;
//     }
//     let height = header.clientHeight;
//     console.log("height", height);
//     container.style.marginTop = height + 16 + "px";
// }

// window.onresize = function() {
//     resetContainerMarginTop();
// }
// window.onload = function() {
//     resetContainerMarginTop();
// }