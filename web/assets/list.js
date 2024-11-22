
let open_select = false; // 开启多选
let selected_url = [];

const pageInfo = {
    current: 1,
    limit: 20,
}

let focus = null;
let list_img = [];
const host = ""
const dialog = document.getElementById("dialog");
const detail = document.getElementById("detail-img");

function calc_img_width(item) {
    let width = item.width;
    if (width > window.innerWidth) {
        // console.log(width);
        width = window.innerWidth * 0.8;
        // item.scaleWidth = width;
    }
    return width;
}

function package_img(list) {
    const box = document.getElementsByClassName("list-box")[0];
    list_img = list;
    let index = 0;
    for (let item of list) {
        const thumb_box = document.createElement("div");
        thumb_box.classList.add("thumb-box")

        const img = document.createElement("img");
        item.index = index++;
        img.classList.add("thumb-img");
        img.classList.add("lazy");
        img.setAttribute("data-src", item.thumb);
        img.setAttribute("loading", "lazy");

        img.addEventListener('click', () => {
            if (open_select) {
                const checkbox = img.parentNode.getElementsByTagName("input")[0];
                checkbox.checked = !checkbox.checked;
                checkbox_change(checkbox, img);
                return;
            }
            const width = calc_img_width(item);
            detail.style.width = 0;
            dialog.showModal();
            detail.setAttribute("src", item.url);
            setTimeout(() => {
                detail.style.width = `${width}px`;
            }, 0);

            console.log(item);
            focus = item;
            dialog.blur();
        })

        thumb_box.appendChild(img);
        box.appendChild(thumb_box);
    }

}

function package_pageInfo(total) {
    const box = document.getElementsByClassName("page-box")[0];
    const currentEle = document.getElementById("current");
    const totalEle = document.getElementById("total");
    totalEle.innerHTML = `共${total}`
    let max_page = Math.floor(total / pageInfo.limit);
    if (total % pageInfo.limit > 0) max_page++;
    currentEle.innerHTML = `${pageInfo.current}/${max_page}`
    pageInfo.max = max_page;
}

function getAll() {
    const box = document.getElementsByClassName("list-box")[0];
    box.innerHTML = "";
    fetch(`${host}/api/getAll?current=${pageInfo.current}&limit=${pageInfo.limit}`)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                package_img(data.data.list)
                package_pageInfo(data.data.total)

                // 实现图片懒加载
                const lazyloadImages = document.querySelectorAll(".lazy");
                var imageObserver = new IntersectionObserver(function (entries, observer) {
                    entries.forEach(function (entry) {
                        if (entry.isIntersecting) {
                            var image = entry.target;
                            image.src = image.dataset.src;
                            image.classList.remove("lazy");
                            imageObserver.unobserve(image);
                        }
                    });
                });

                lazyloadImages.forEach(function (image) {
                    imageObserver.observe(image);
                });

            } else {
                toast("error", data.msg);
            }
        })
        .catch(err => console.log('Request Failed', err));
}

document.addEventListener('DOMContentLoaded', getAll);

function selected_picture(checkbox, img) {
    const current_url = img.getAttribute("src");
    selected_url.push(current_url);
    checkbox.checked = true;
}
function unselected_picture(checkbox, img) {
    const current_url = img.getAttribute("src");
    let index = -1;
    for (const url_tmp of selected_url) {
        index++;
        if (url_tmp === current_url) {
            selected_url.splice(index, 1); // 删掉
            checkbox.checked = false;
            break
        }
    }
}

function checkbox_change(checkbox, img) {
    if (!checkbox && !img) {
        return;
    }
    const current_url = img.getAttribute("src");
    if (checkbox.checked) {
        selected_picture(checkbox, img);
    } else {
        unselected_picture(checkbox, img);
    }
    console.log(selected_url);
}

window.addEventListener('load', () => {
    document.getElementById("pre_page").addEventListener("click", () => {
        if (pageInfo.current - 1 <= 0) {
            return
        }
        pageInfo.current -= 1;
        getAll();
    })
    document.getElementById("next_page").addEventListener("click", () => {
        if (pageInfo.current + 1 > pageInfo.max) {
            return
        }
        pageInfo.current += 1;
        getAll();
    })
    const select_pic = document.getElementById("select-pic");
    const select_del = document.getElementById("select-del");
    const select_all = document.getElementById("select-all");
    select_pic.addEventListener("click", () => {
        open_select = !open_select;
        if (open_select) {
            select_pic.innerHTML = "取消选择"
            select_del.style.display = 'inline-block';
            select_all.style.display = 'inline-block';
        } else {
            select_del.style.display = 'none';
            select_all.style.display = 'none';
            select_pic.innerHTML = "选择"
            selected_url = [];
        }

        const box = document.getElementsByClassName("list-box")[0];
        const thumb_boxs = box.getElementsByClassName("thumb-box");
        for (const thumb_box of thumb_boxs) {
            if (open_select) {
                const img = thumb_box.getElementsByTagName("img")[0];
                const checkbox = document.createElement("input");
                checkbox.classList.add("select-checkbox")
                checkbox.setAttribute("type", "checkbox")
                checkbox.addEventListener('click', () => {
                    checkbox_change(checkbox, img);
                })
                thumb_box.appendChild(checkbox);
            } else {
                for (const checkbox of thumb_box.getElementsByClassName("select-checkbox")) {
                    thumb_box.removeChild(checkbox);
                }
            }
        }
    })

    select_del.addEventListener("click", () => {
        del_pic(selected_url)
    })
    select_all.addEventListener("click", () => {
        const box = document.getElementsByClassName("list-box")[0];
        const thumb_boxs = box.getElementsByClassName("thumb-box");
        if (thumb_boxs.length == selected_url.length) {
            selected_url = [];
            for (const thumb_box of thumb_boxs) {
                const checkbox = thumb_box.getElementsByClassName("select-checkbox")[0];
                // checkbox.removeAttribute("checked");
                checkbox.checked = false;
            }
            console.log(selected_url);
            return;
        }
        selected_url = [];
        for (const thumb_box of thumb_boxs) {
            const checkbox = thumb_box.getElementsByClassName("select-checkbox")[0];
            // checkbox.setAttribute("checked", true);
            checkbox.checked = true;
            const img = thumb_box.getElementsByTagName("img")[0];
            selected_url.push(img.getAttribute("src"));
        }
        console.log(selected_url);
    })

    // 点击遮罩关闭弹窗
    dialog.addEventListener('click', function (evt) {
        if (evt.target.nodeName === 'DIALOG') this.close()
    })

    document.addEventListener('keydown', evt => {
        const code = evt.key;
        const open = dialog.getAttribute("open") != null;
        if (open) {
            if (code === 'ArrowLeft') { // left
                pre_pic();
            }
            else if (code == 'ArrowRight') { // right
                next_pic();
            }

        }
    });

    initThemeBtn();
})


/* 下面是控制条的代码 */

function del_pic(urls) {
    let final_url = "";
    if (confirm(`确定删除？${urls ? '共' + urls.length + '条' : ''}`)) {
        if (urls != null) {
            if (urls.length === 0) {
                toast("warn", "请先选择")
                return;
            }
            final_url = `${host}/api/deleteAll?url=${JSON.stringify(urls)}`
        }
        else {
            final_url = `${host}/api/delete?url=${focus.thumb}`
        }

        fetch(final_url)
            .then(response => response.json())
            .then(data => {
                if (data.success) {
                    console.log("删除成功");
                    location.reload();
                } else {
                    toast("error", data.msg);
                }
            })
            .catch(err => console.log('Request Failed', err));
    }
}

function copy_url() {
    if (!focus) {
        return;
    }
    copyUrl(focus.url);
}

function download_pic() {
    const download_btn = document.getElementById("download-pic");
    download_btn.setAttribute("href", focus.url);
}

let scale_factor = 1;
function scale_pic(type) {
    let width = 0;
    let old_scale_factor = scale_factor;
    switch (type) {
        case 1:
            scale_factor = 1;
            width = calc_img_width(focus);
            break;
        case 2:
            scale_factor += 0.2;
            width = calc_img_width(focus) * scale_factor;
            break;
        case 3:
            scale_factor -= 0.2;
            width = calc_img_width(focus) * scale_factor;
            break;
    }
    if (width < 100) {
        scale_factor = old_scale_factor
        return;
    }

    detail.style.width = `${width}px`;
    // console.log(detail.style.width);
}

function show_origin_pic() {
    window.open(focus.url);
}

function change_pic() {
    // console.log(focus);
    const detail = document.getElementById("detail-img");
    detail.setAttribute("src", focus.url);
    const width = calc_img_width(focus);
    detail.style.width = `${width}px`
    dialog.blur();
}

function pre_pic() {
    const pre_index = focus.index - 1;
    if (pre_index < 0) {
        toast("warn", "已经是第一张了~");
        return;
    }
    focus = list_img[pre_index];
    change_pic();
}

function next_pic() {
    const next_index = focus.index + 1;
    if (next_index >= list_img.length) {
        toast("warn", "已经是最后一张了~");
        return;
    }
    focus = list_img[next_index];
    change_pic();
}

function close_pic() {
    dialog.close();
}