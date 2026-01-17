// 上传区域拖拽交互
const uploadZone = document.getElementById('upload-zone');
const uploadInput = document.getElementById('upload');
const previewBox = document.getElementById('preview-box');
const previewCount = document.getElementById('preview-count');
const clearBtn = document.getElementById('clear-btn');
const qualityDisplay = document.getElementById('quality-display');
const qualityHint = document.getElementById('quality-hint');
const qualityRange = document.getElementById('quality-range');

// 拖拽状态
if (uploadZone) {
    ['dragenter', 'dragover'].forEach(eventName => {
        uploadZone.addEventListener(eventName, (e) => {
            e.preventDefault();
            e.stopPropagation();
            uploadZone.classList.add('drag-over');
        });
    });

    ['dragleave', 'drop'].forEach(eventName => {
        uploadZone.addEventListener(eventName, (e) => {
            e.preventDefault();
            e.stopPropagation();
            uploadZone.classList.remove('drag-over');
        });
    });
}

// 更新预览数量
function updatePreviewCount() {
    const count = previewBox ? previewBox.children.length : 0;
    if (previewCount) previewCount.textContent = count;
    if (clearBtn) clearBtn.style.display = count > 0 ? 'block' : 'none';
}

// 清空预览
if (clearBtn) {
    clearBtn.addEventListener('click', () => {
        if (previewBox) previewBox.innerHTML = '';
        updatePreviewCount();
    });
}

// 点击上传区域时触发文件选择
const uploadContent = document.querySelector('.upload-content');
uploadContent?.addEventListener('click', (e) => {
    // 阻止事件冒泡，避免重复触发
    e.stopPropagation();
    uploadInput?.click();
});

// 选择上传文件
uploadInput?.addEventListener('change', (evt) => {
    const filesList = uploadInput.files;
    if (filesList.length === 0) {
        toast('warn', '没有选择任何文件');
        return;
    }
    handleFiles(filesList);
});

// 处理文件列表
function handleFiles(files) {
    for (const file of files) {
        const reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = () => {
            addPreviewCard(file, reader.result);
        };
    }
}

// 添加预览卡片
function addPreviewCard(file, dataUrl) {
    const preview = document.createElement('div');
    preview.classList.add('preview');
    preview.style.animationDelay = `${previewBox.children.length * 0.05}s`;

    const img = document.createElement('img');
    img.classList.add('preview-img');
    img.src = dataUrl;
    img.setAttribute('f-name', file.name);
    img.setAttribute('f-type', file.type);
    img.setAttribute('f-size', file.size);
    img.setAttribute('f-modify', file.lastModified);

    const fileName = document.createElement('div');
    fileName.classList.add('file-name');
    fileName.textContent = file.name;

    const delBtn = document.createElement('button');
    delBtn.classList.add('delete-btn');
    delBtn.innerHTML = '×';
    delBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        preview.remove();
        updatePreviewCount();
    });

    preview.appendChild(img);
    preview.appendChild(fileName);
    preview.appendChild(delBtn);
    previewBox.appendChild(preview);
    updatePreviewCount();
}

// 点击上传
document.getElementById('upload-btn')?.addEventListener('click', () => {
    const imgs = previewBox?.querySelectorAll('img');
    if (!imgs || imgs.length === 0) {
        toast('info', '请先选择图片');
        return;
    }

    let quality = parseInt(qualityRange?.value || '40', 10);
    if (isNaN(quality)) quality = 40;

    const url = '/api/upload';

    imgs.forEach((img, index) => {
        if (img.getAttribute('success')) return;

        const parentNode = img.parentNode;

        // 清除旧的加载状态
        parentNode.querySelectorAll('.loading, .progress-overlay').forEach(el => el.remove());

        const fileType = img.getAttribute('f-type');
        const fileName = img.getAttribute('f-name');
        const fileSize = img.getAttribute('f-size');
        const fileData = img.getAttribute('src');

        // 添加进度覆盖层
        const progressOverlay = document.createElement('div');
        progressOverlay.classList.add('progress-overlay');
        progressOverlay.innerHTML = `
            <svg class="progress-ring" viewBox="0 0 44 44">
                <circle class="progress-ring-bg" cx="22" cy="22" r="20"/>
                <circle class="progress-ring-fill" cx="22" cy="22" r="20"/>
            </svg>
        `;
        parentNode.appendChild(progressOverlay);

        const formData = {
            name: fileName,
            size: fileSize,
            type: fileType,
            quality: quality,
            data: fileData,
        };
        const queryString = Object.keys(formData).map(key => `${key}=${formData[key]}`).join('&');

        fetch(url, {
            method: 'post',
            body: queryString,
        })
            .then(response => response.json())
            .then(json => {
                img.setAttribute('success', json.success);
                progressOverlay.remove();

                if (json.success) {
                    toast('success', json.msg);

                    const imgName = parentNode.querySelector('.file-name');
                    imgName.classList.add('success');
                    imgName.title = '点击复制链接';

                    const resUrl = json.data.url;
                    const thumb = json.data.thumb;
                    if (thumb && resUrl) {
                        imgName.addEventListener('click', () => copyUrl(resUrl));
                        img.src = thumb;
                    }
                } else {
                    toast('error', json.msg);
                    const imgName = parentNode.querySelector('.file-name');
                    imgName.classList.add('error');
                    imgName.textContent = json.msg || '上传失败';
                }
            })
            .catch(err => {
                console.error('Upload failed:', err);
                progressOverlay.remove();
                toast('error', '上传失败');
            });
    });
});

// 质量滑块
window.addEventListener('load', () => {
    initThemeBtn();
    updatePreviewCount();

    qualityRange?.addEventListener('input', (evt) => {
        const value = parseInt(evt.target.value, 10);
        qualityDisplay.textContent = `${value}%`;

        if (value >= 100) {
            qualityHint.innerHTML = `
                <span class="compress-badge original">原图</span>
                保持原始格式和质量
            `;
        } else {
            qualityHint.innerHTML = `
                <span class="compress-badge">WebP 压缩</span>
                输出为 WebP 格式，体积更小
            `;
        }
    });
});
