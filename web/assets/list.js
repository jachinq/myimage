
let open_select = false; // 开启多选
let selected_url = [];

const pageInfo = {
    currentPage: 1,      // 当前页码
    limit: 20,           // 每次加载数量
    total: 0,            // 总数量
    loading: false,      // 加载状态标志
    hasMore: true        // 是否有更多数据
}

let focus = null;
let list_img = [];
const host = ""
const overlay = document.getElementById("preview-overlay");
const detail = document.getElementById("detail-img");

// 拖拽状态管理
let isDragging = false;
let dragStart = { x: 0, y: 0 };
let imagePos = { x: 0, y: 0 };
let rafId = null; // requestAnimationFrame ID

function calc_img_width(item) {
    let width = item.width;
    if (width > window.innerWidth * 0.8) {
        width = window.innerWidth * 0.8;
    }
    return width;
}

// 拖拽功能初始化
function initDrag() {
    // 使用 Pointer Events API 统一处理鼠标、触摸、笔输入
    detail.addEventListener('pointerdown', startDrag);
    detail.addEventListener('pointermove', drag);
    detail.addEventListener('pointerleave', endDrag);
    detail.addEventListener('pointerup', endDrag);
    detail.addEventListener('pointercancel', endDrag);
    detail.style.cursor = 'grab';
    detail.style.touchAction = 'none'; // 防止页面滚动干扰拖拽
}

function startDrag(e) {
    // 阻止默认行为（如页面滚动）
    e.preventDefault();

    isDragging = true;
    dragStart = {
        x: e.clientX - imagePos.x,
        y: e.clientY - imagePos.y
    };
    detail.style.cursor = 'grabbing';

    // 将 pointermove 监听提升到 document，确保拖拽不丢失
    document.addEventListener('pointermove', drag);

    // 在 document 上监听 pointerup，确保拖拽结束时能正确捕获
    document.addEventListener('pointerup', endDrag);
}

function drag(e) {
    if (!isDragging) return;

    // 阻止默认滚动行为
    e.preventDefault();

    imagePos = {
        x: e.clientX - dragStart.x,
        y: e.clientY - dragStart.y
    };

    // 使用 requestAnimationFrame 批量更新，避免每帧多次 DOM 操作
    if (!rafId) {
        rafId = requestAnimationFrame(() => {
            detail.style.transform = `translate(${imagePos.x}px, ${imagePos.y}px) scale(${scale_factor})`;
            rafId = null;
        });
    }
}

function endDrag() {
    isDragging = false;
    detail.style.cursor = 'grab';

    // 清理 document 上的监听器
    document.removeEventListener('pointermove', drag);
    document.removeEventListener('pointerup', endDrag);

    // 确保最后一次位置更新并清理 rafId
    if (rafId) {
        cancelAnimationFrame(rafId);
        rafId = null;
    }
}

function appendImages(list) {
    const box = document.getElementsByClassName("list-box")[0];
    const startIndex = list_img.length;

    // 将新数据追加到现有数据
    list_img = list_img.concat(list);

    let index = startIndex;
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

            // 重置位置和缩放
            imagePos = { x: 0, y: 0 };
            scale_factor = 1;
            detail.style.transform = `translate(0px, 0px) scale(1)`;

            // 显示全屏遮罩层
            overlay.classList.add('active');
            detail.setAttribute("src", item.url);

            console.log(item);
            focus = item;
        })

        thumb_box.appendChild(img);
        box.appendChild(thumb_box);
    }

    // 应用懒加载
    const lazyloadImages = box.querySelectorAll(".lazy:not(.observed)");
    var imageObserver = new IntersectionObserver(function (entries, observer) {
        entries.forEach(function (entry) {
            if (entry.isIntersecting) {
                var image = entry.target;
                image.src = image.dataset.src;
                image.classList.remove("lazy", "observed");
                observer.unobserve(image);
            }
        });
    });

    lazyloadImages.forEach(function (image) {
        image.classList.add("observed");
        imageObserver.observe(image);
    });
}

function updatePageInfo(total) {
    const totalEle = document.getElementById("total");
    totalEle.innerHTML = `已加载 ${list_img.length}/${total}`;
    pageInfo.total = total;
}

function loadMore() {
    if (pageInfo.loading || !pageInfo.hasMore) return;

    pageInfo.loading = true;
    showLoadingState();

    fetch(`${host}/api/getList?current=${pageInfo.currentPage}&limit=${pageInfo.limit}`)
        .then(response => response.json())
        .then(data => {
            if (data.success) {
                appendImages(data.data.list);
                pageInfo.currentPage += 1;
                pageInfo.hasMore = list_img.length < data.data.total;
                updatePageInfo(data.data.total);

                if (!pageInfo.hasMore) {
                    showEndState();
                }
            } else {
                toast("error", data.msg);
            }
            pageInfo.loading = false;
            hideLoadingState();
        })
        .catch(err => {
            console.log('Request Failed', err);
            pageInfo.loading = false;
            hideLoadingState();
        });
}

function showLoadingState() {
    let loader = document.getElementById('infinite-scroll-loader');
    if (!loader) {
        loader = document.createElement('div');
        loader.id = 'infinite-scroll-loader';
        loader.className = 'loading';
        loader.innerHTML = '<div class="loading-spinner"></div>';
        document.querySelector('.container').appendChild(loader);
    }
    loader.style.display = 'flex';
}

function hideLoadingState() {
    const loader = document.getElementById('infinite-scroll-loader');
    if (loader) {
        loader.style.display = 'none';
    }
}

function showEndState() {
    let endMsg = document.getElementById('infinite-scroll-end');
    if (!endMsg) {
        endMsg = document.createElement('div');
        endMsg.id = 'infinite-scroll-end';
        endMsg.className = 'end-message';
        endMsg.innerHTML = '<p>没有更多图片了</p>';
        document.querySelector('.container').appendChild(endMsg);
    }
    endMsg.style.display = 'block';
}

function initInfiniteScroll() {
    const sentinel = document.createElement('div');
    sentinel.id = 'scroll-sentinel';
    sentinel.style.height = '1px';
    document.querySelector('.container').appendChild(sentinel);

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting && pageInfo.hasMore && !pageInfo.loading) {
                loadMore();
            }
        });
    }, { threshold: 0 });

    observer.observe(sentinel);
}

document.addEventListener('DOMContentLoaded', () => {
    initInfiniteScroll();
    loadMore();
});

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
                checkbox.setAttribute("name", "select-pic")
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


    // 初始化拖拽功能
    initDrag();

    document.addEventListener('keydown', evt => {
        const code = evt.key;
        const open = overlay.classList.contains('active');
        if (open) {
            if (code === 'Escape') { // ESC 关闭
                close_pic();
            } else if (code === 'ArrowLeft') { // 左箭头 上一张
                pre_pic();
            } else if (code == 'ArrowRight') { // 右箭头 下一张
                next_pic();
            }
        }
    });
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
    let old_scale_factor = scale_factor;
    switch (type) {
        case 1:
            scale_factor = 1;
            break;
        case 2:
            scale_factor += 0.2;
            break;
        case 3:
            scale_factor -= 0.2;
            break;
    }

    // 限制最小缩放比例
    if (scale_factor < 0.2) {
        scale_factor = old_scale_factor;
        return;
    }

    // 只使用 transform 处理缩放和拖拽位置
    detail.style.transform = `translate(${imagePos.x}px, ${imagePos.y}px) scale(${scale_factor})`;
}

function show_origin_pic() {
    window.open(focus.url);
}

function change_pic() {
    // console.log(focus);
    const detail = document.getElementById("detail-img");
    detail.setAttribute("src", focus.url);
    // 重置拖拽位置和缩放
    imagePos = { x: 0, y: 0 };
    scale_factor = 1;
    detail.style.transform = `translate(0px, 0px) scale(1)`;
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
    overlay.classList.remove('active');
}