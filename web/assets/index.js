    // 选择上传文件
    document.getElementById("upload")?.addEventListener('change', (evt) => {
        console.log(evt);
        var filesList = document.querySelector('#upload').files;
        if (filesList.length == 0) { //如果取消上传，则上传文件的长度为0
            toast('warn', "没有上传任何文件");
            return;
        }

        // 有文件上传，处理为预览图
        for (const file of filesList) {
            const reader = new FileReader();
            console.log(file);
            reader.readAsDataURL(file);
            reader.onload = () => {
                const preview_box = document.getElementsByClassName("preview-box")[0];

                const preview = document.createElement("div");
                preview.classList.add("preview")

                const img = document.createElement("img");
                img.classList.add("preview-img");
                img.setAttribute("src", reader.result);
                img.setAttribute("f-name", file.name);
                img.setAttribute("f-type", file.type);
                img.setAttribute("f-size", file.size);
                img.setAttribute("f-modify", file.lastModified);

                const img_name = document.createElement("div");
                img_name.innerHTML = file.name;
                img_name.classList.add("file-name");

                const del_btn = document.createElement("div");
                del_btn.classList.add("delete-btn");
                const x = document.createElement("span");
                x.innerHTML = "x";
                // const x = document.getElementById("close").cloneNode();
                // x.style.display = "block"
                del_btn.appendChild(x);
                del_btn.addEventListener('click', () => {
                    preview_box.removeChild(preview);
                })

                preview.appendChild(img);
                preview.appendChild(img_name);
                preview.appendChild(del_btn);
                preview_box.appendChild(preview);
            }
        }

    });

    // 点击上传
    document.getElementById("upload-btn")?.addEventListener('click', () => {
        const imgs = document.getElementsByClassName("preview-box")[0].getElementsByTagName("img");
        if (!imgs || imgs.length == 0) {
            toast("info", "先选择图片")
            return
        }

        let quality = document.getElementById("quality").innerHTML;
        quality =  Number(quality); // 验证质量是否为数字
        if (isNaN(quality)) {
            toast("warn", "质量不是数字，默认使用40进行上传")
            quality = 40; // 默认为40
        }
        const url = "/api/upload";

        for (const img of imgs) {
            const success = img.getAttribute("success");
            if (success) {
                continue;
            }
            const parentNode = img.parentNode;
            // 避免重试的时候，还有旧的loading
            const loadings = parentNode.getElementsByClassName("loading");
            if (loadings) {
                console.log(loadings);
                for (const loadingTmp of loadings) {
                    parentNode.removeChild(loadingTmp);
                }
            }

            const file_type = img.getAttribute("f-type");
            const file_name = img.getAttribute("f-name");
            const file_size = img.getAttribute("f-size");
            const file_data = img.getAttribute("src");

            const loading = document.createElement("div");
            loading.classList.add("loading");
            const loading_text = document.createElement("div");
            loading_text.classList.add("loading-text")
            loading.appendChild(loading_text);
            parentNode.appendChild(loading)

            const formData = {
                name: file_name,
                size: file_size,
                type: file_type,
                quality: quality,
                data: file_data, // base64 放最后
            }
            // change formData to query string
            const queryString = Object.keys(formData).map(key => `${key}=${formData[key]}`).join('&');
            // console.log(formData, queryString);

            fetch(url, {
                method: "post",
                body: queryString,
            })
                .then(response => response.json())
                .then(json => {
                    console.log(json);
                    img.setAttribute("success", json.success);
                    parentNode.removeChild(loading)

                    if (json.success) {
                        toast("success", json.msg);

                        const img_name = parentNode.getElementsByClassName("file-name")[0];
                        img_name.classList.add("success");
                        img_name.setAttribute("title", "复制链接")

                        const url = json.data.url;
                        const thumb = json.data.thumb;
                        if (thumb && url) {
                            img_name.addEventListener('click', () => copyUrl(url));
                            img.src = thumb; // 替换为服务器地址
                        }
                    } else {
                        toast("error", json.msg);
                        const result = document.createElement("span");
                        result.classList.add("error");
                        result.innerHTML = json.msg;
                        parentNode.appendChild(result);
                    }
                })
                .catch(err => console.log('Request Failed', err));
        }

    })

    window.addEventListener('load', () => {
        initThemeBtn();

        const quality = document.getElementById("quality");
        const qualityUnit = document.getElementById("quality-unit");
        document.getElementById("quality-range")?.addEventListener('input', (evt) => {
            const value = evt.target.value;
            // console.log(value);
            quality.innerHTML = value;
            if (value >= 100 || value < 10) {
                qualityUnit.innerHTML = "% ✅原图上传";
            } else {
                qualityUnit.innerHTML = "% ❎压缩上传";
            }
        })
    })
