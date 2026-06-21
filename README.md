<img width="200px" src="public/icon.svg" align="left"/>

# OnePot

> 📄 **OnePot** 是 [Pot](https://github.com/pot-app/pot-desktop) 的一个 Fork，在保留 Pot 全部翻译/OCR/语音合成功能的基础上，专注于**论文文献的便捷解析、搜索与下载**。

> **Pot** 是: 🌈 一个跨平台的划词翻译软件 ([项目主页](https://pot-app.com/))

![License](https://img.shields.io/github/license/pot-app/pot-desktop.svg)
![Tauri](https://img.shields.io/badge/Tauri-1.6.8-blue?logo=tauri)
![JavaScript](https://img.shields.io/badge/-JavaScript-yellow?logo=javascript&logoColor=white)
![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white)
![MacOS](https://img.shields.io/badge/-macOS-black?&logo=apple&logoColor=white)
![Linux](https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white)

<br/>
<hr/>

<div align="center">

<!-- <h3>中文 | <a href='./README_EN.md'>English</a> | <a href='./README_KR.md'> 한글 </a></h3> -->

<!-- <table>
<tr><td> <img src="asset/1.png"> <td> <img src="asset/2.png"> <td> <img src="asset/3.png">
</table> -->

# 目录

</div>

- [简介](#简介)
- [文献流水线](#文献流水线)
- [功能特性](#功能特性)
- [安装与编译](#安装与编译)
- [配置说明](#配置说明)
- [注意事项](#注意事项)
- [感谢](#感谢)

<div align="center">

# 简介

</div>

OnePot 的前身是 [OneShot](https://github.com/Raiscies/oneshot). 它能够通过选中参考文献文本, 自动解析并下载对应 PDF. 但由于该项目存在诸多问题，遂基于 Pot 完全重写，在保留 Pot 全部功能的同时，实现 Oneshot 的所有功能. 

<div align="center">

# 文献流水线

</div>

OnePot 的核心新增功能：将剪贴板中的参考文献文本自动转换为结构化的论文信息，并可一键下载 PDF。

## 使用步骤

1. **选中文本**：在 PDF 阅读器、网页等处选中参考文献条目（支持复制多条）。
2. **按下快捷键**：按下设置中配置好的"划词识别文献"快捷键，弹出文献卡片窗口。
3. **自动解析**：后端调用 [AnyStyle](https://github.com/inukshuk/anystyle) 解析引文格式，提取标题、作者、年份等信息。
4. **自动补全**：通过 [Semantic Scholar API](https://api.semanticscholar.org/) 补全 DOI、摘要、引用次数、期刊/会议名等元数据。
5. **一键下载**：点击卡片上的下载按钮，支持多家出版商（ACM、IEEE、Springer 等）的 PDF 直接下载（此功能需要你的网络环境已经越过付费墙. APP通过 [Cloudflare Bypasser](https://github.com/Raiscies/cloudflare-bypass-python) 代理绕过防护）。
6. **自动打开**：可在设置中开启下载后自动打开 PDF，或自动在浏览器中打开论文页面。


<div align="center">

# 功能特性

</div>

## 🆕 OnePot 新增


| 功能 | 说明 |
|------|------|
| 📚 引文解析 | 基于 [AnyStyle](https://github.com/inukshuk/anystyle)（Ruby），支持多种引用格式 |
| 🔍 论文补全 | 自动查询 [Semantic Scholar](https://www.semanticscholar.org/)，获取 DOI、摘要、引用次数 |
| 📄 PDF 下载 | 配置驱动，支持 8+ 学术出版商，自动绕过 Cloudflare 防护 |

## ✅ 继承自 Pot

OnePot 保留了 Pot 的全部功能, 详细说明请参阅 [Pot 官方文档](https://pot-app.com/)。

## 🚧 TODO

- [-] 包括 CCF 等级在内的各类期刊排名显示 (CCF已完成)
- [ ] 尝试脱离部分外部依赖
- [ ] 自动运行 CloudflareBypasser（无需手动启动）
- [ ] 扩充解析内容范围（关键词, URL等更多形式）
- [ ] 支持更多学术出版商网站下载
- [ ] 支持更多搜索源与用户自定义搜索源（如 Sci-Hub）
- [ ] 其它更多更多优化!

<div align="center">

# 安装与编译

</div>

## 手动编译

### 环境要求

| 工具 | 最低版本 | 用途 |
|------|---------|------|
| Node.js | >= 18.0.0 | 前端构建 |
| pnpm | >= 8.5.0 | 包管理器 |
| Rust | >= 1.80.0 | Tauri 后端 |
| Ruby | >= 4.0.5 | AnyStyle 引文解析 |
| Python | >= 3.12 | Cloudflare Bypasser（可选，PDF 下载需要） |

### 开始编译

1. **Clone 仓库**

    ```bash
    git clone https://github.com/Raiscies/onepot.git
    cd onepot
    ```

2. **安装前端依赖**

    ```bash
    pnpm install
    ```

3. **安装系统依赖（仅 Linux）**

    ```bash
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev patchelf libxdo-dev libxcb1 libxrandr2 libdbus-1-3
    ```

4. **安装 [Ruby](https://www.ruby-lang.org/) 和 [AnyStyle](https://github.com/inukshuk/anystyle)（文献解析）**

    ```bash
    # Windows
    winget install RubyInstallerTeam.RubyWithDevKit.4.0.5
    # 重启终端后
    gem install anystyle

    # macOS
    brew install ruby
    gem install anystyle

    # Linux
    sudo apt-get install ruby ruby-dev
    gem install anystyle
    ```

5. **开发调试**

    ```bash
    pnpm tauri dev
    ```

6. **打包构建**

    ```bash
    pnpm tauri build
    ```

7. [Cloudflare 绕过](https://github.com/sarperavci/CloudflareBypassForScraping): 文献下载功能假定你的网络环境已经越过付费墙 (例如校园网). 为了能够顺利下载文献, 需要一个 Cloudflare Bypasser 路由服务: 
    ```bash
        git clone https://github.com/sarperavci/CloudflareBypassForScraping.git
        cd CloudflareBypassForScraping
        pip install -r server_requirements.txt
        python server.py --host 127.0.0.1 --port 8000
        # 此时路由服务已经启动, 需要常驻使 OnePot 能够访问
    ```
> 有关 Cloudflare Bypasser 的更多细节请参考它的官网说明.

<div align="center">

# 配置说明

</div>

以下配置项可在 **偏好设置 → 文献识别/文献下载/热键设置** 中找到：

| 配置项 | 说明 |
|-------|------|
| 划词识别文献快捷键| 触发文献解析的全局快捷键 |
| 自动下载计数 | 结果 ≤ N 条时自动下载；0 关闭 |
| 自动打开 PDF | 下载完成后自动打开 |
| 自动打开论文页| 下载失败时在浏览器中打开论文链接 |
| CF Bypass 主机| Cloudflare 绕过代理地址 |
| CF Bypass 端口| 代理端口 |
| 搜索引擎 | 点击搜索按钮打开的搜索引擎 |


<div align="center">

# 注意事项

</div>
关于Cloudflare绕过服务: 

- 绕过并非100%能够成功, 其可用性比较玄学, 你当前的网络与系统环境对解决 Cloudflare 的 anti-bot challenges都有微妙的影响. 一般来说, 最好确保 `challenges.cloudflare.com` 这个域名是直连的. 换句话说就是, 最好不要在绕过服务后面加任何其他的代理. 如果绕过失败, 可以检查一下是不是系统代理把这个域名给转发了. 
- 如果必须使用网络代理以越过付费墙 (如需要从外部网代理到校园网环境), 考虑在校园网内部环境部署绕过服务而不是在本机部署, 然后配置系统代理转发绕过服务, 并在APP设置中配置绕过服务的主机和端口号. 
- 这个项目在第一次响应请求时会自动安装大约200M的CloakBrowser, 响应可能较慢; 同时在每次Cookies Cache过期后, 需要重新解决challenges, 响应也会比较慢.

<div align="center">

# 感谢

</div>

- [Pot](https://github.com/pot-app/pot-desktop) — 上游项目，提供翻译/OCR/插件系统等全部基础功能
- [AnyStyle](https://github.com/inukshuk/anystyle) — 文献引用解析引擎
- [Semantic Scholar](https://www.semanticscholar.org/) — 论文学术元数据 API
- [Tauri](https://github.com/tauri-apps/tauri) — Rust 驱动的跨平台 GUI 框架
- [Cloudflare Bypass](https://github.com/sarperavci/CloudflareBypassForScraping) - Cloudflare 绕过工具

</div>
