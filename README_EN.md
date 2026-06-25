<img width="200px" src="public/icon.svg" align="left"/>

# OnePot

> 📄 **OnePot** is a fork of [Pot](https://github.com/pot-app/pot-desktop). While retaining all of Pot's translation / OCR / TTS capabilities, it focuses on **convenient parsing, searching, and downloading of academic papers**.

> **Pot** is: 🌈 a cross-platform text translation tool ([Homepage](https://pot-app.com/))

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

<h3><a href='./README.md'>中文</a> | English </h3>

# Table of Contents

</div>

- [Introduction](#introduction)
- [Paper Pipeline](#paper-pipeline)
- [Features](#features)
- [Installation & Building](#installation--building)
- [Configuration](#configuration)
- [Notes](#notes)
- [Acknowledgements](#acknowledgements)

<div align="center">

# Introduction

</div>

OnePot traces its origin to [OneShot](https://github.com/Raiscies/oneshot). OneShot could parse reference strings and download the corresponding PDFs, but suffered from numerous issues. OnePot was therefore rewritten entirely on top of Pot, preserving all of Pot's functionality while realizing all of OneShot's capabilities.

<div align="center">

# Paper Pipeline

</div>

OnePot's core new feature: automatically converts bibliography text from the clipboard into structured paper metadata, with one-click PDF download.

## Quick Walkthrough

1. **Select text**: highlight one or more reference entries in a PDF reader, web page, etc.
2. **Press the hotkey**: use the configured "Recognize Citation" shortcut to open the paper card window.
3. **Auto-parse**: the backend calls [AnyStyle](https://github.com/inukshuk/anystyle) to parse citation formats and extract title, authors, year, etc.
4. **Auto-enrich**: metadata (DOI, abstract, citation count, journal/conference name, etc.) is completed via the [Semantic Scholar API](https://api.semanticscholar.org/).
5. **One-click download**: click the download button on the card. PDF downloads are supported for 8+ academic publishers (requires network access that is already behind the paywall. The app proxies requests through [Cloudflare Bypasser](https://github.com/sarperavci/CloudflareBypassForScraping) to bypass anti-bot protection).
6. **Auto-open**: optionally auto-open the downloaded PDF, or open the paper page in a browser.

<div align="center">

# Features

</div>

## 🆕 New in OnePot

| Feature | Description |
|---------|------------|
| 📚 Citation Parsing | Powered by [AnyStyle](https://github.com/inukshuk/anystyle) (Ruby); supports a wide variety of citation formats |
| 🔍 Paper Enrichment | Queries [Semantic Scholar](https://www.semanticscholar.org/) for DOI, abstract, citation count |
| 📄 PDF Download | Config-driven, supports 8+ academic publishers, automatically bypasses Cloudflare protection |

See [SUPPORTED_PUBLISHERS.md](./SUPPORTED_PUBLISHERS.md) for supported publisher sites.

## ✅ Inherited from Pot

OnePot retains all of Pot's features. For details, refer to the [Pot official documentation](https://pot-app.com/).

## 🚧 TODO

- [x] Journal ranking display including CCF ranks (partial, CCF completed)
- [ ] Reduce external dependencies where possible
- [ ] Auto-start CloudflareBypasser (no manual launch required)
- [ ] Expand recognized content types (keywords, URLs, etc.)
- [ ] Support more academic publisher sites
- [ ] Support more search sources and user-custom sources (e.g., Sci-Hub)
- [ ] Many more improvements!

<div align="center">

# Installation & Building

</div>

## Build from Source

### Requirements

| Tool | Minimum Version | Purpose |
|------|-----------------|---------|
| Node.js | >= 18.0.0 | Frontend build |
| pnpm | >= 8.5.0 | Package manager |
| Rust | >= 1.80.0 | Tauri backend |
| Ruby | >= 4.0.5 | AnyStyle citation parsing |
| Python | >= 3.12 | Cloudflare Bypasser (optional — required for PDF download) |

### Steps

1. **Clone the repository**

    ```bash
    git clone https://github.com/Raiscies/onepot.git
    cd onepot
    ```

2. **Install frontend dependencies**

    ```bash
    pnpm install
    ```

3. **Install system dependencies (Linux only)**

    ```bash
    sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev patchelf libxdo-dev libxcb1 libxrandr2 libdbus-1-3
    ```

4. **Install [Ruby](https://www.ruby-lang.org/) and [AnyStyle](https://github.com/inukshuk/anystyle) (citation parsing)**

    ```bash
    # Windows
    winget install -e --id RubyInstallerTeam.Ruby.4.0

    # macOS
    brew install ruby

    # Linux
    sudo apt-get install ruby-full
    ```

    install anystyle gem after restarting your terminal
    ```bash
    gem install anystyle
    ```

5. **Development**

    ```bash
    pnpm tauri dev
    ```

6. **Production build**

    ```bash
    pnpm tauri build
    ```

7. **[Cloudflare bypass](https://github.com/sarperavci/CloudflareBypassForScraping)**: the paper download feature assumes your network is already behind the paywall (e.g., campus network). To download papers, you need a running Cloudflare Bypasser service:

    ```bash
        git clone https://github.com/sarperavci/CloudflareBypassForScraping.git
        cd CloudflareBypassForScraping
        pip install -r server_requirements.txt
        python server.py --host 127.0.0.1 --port 8000
        # The bypass service must run persistently for OnePot to access it
    ```

> For more details on Cloudflare Bypasser, see its official documentation.

<div align="center">

# Configuration

</div>

The following settings can be found in **Preferences → Paper Recognition / Paper Download / Hotkey Settings**:

| Setting | Description |
|---------|------------|
| Recognize Citation hotkey | Global shortcut to trigger citation parsing |
| Auto-download threshold | Auto-download when results ≤ N; set to 0 to disable |
| Auto-open PDF | Automatically open PDF after download |
| Auto-open paper page | Open paper link in browser when download fails |
| CF Bypass host | Cloudflare bypass proxy address |
| CF Bypass port | Proxy port |
| Search engine | Search engine opened by the search button |

<div align="center">

# Notes

</div>

Regarding the Cloudflare bypass service:

- Bypass is not 100% reliable — its availability can be finicky. Your network and system environment have subtle effects on solving Cloudflare's anti-bot challenges. Generally, it is best to ensure the domain `challenges.cloudflare.com` is accessed directly. In other words, avoid adding any additional proxy behind the bypass service. If bypass fails, check whether your system proxy is forwarding this domain.
- If a network proxy is required to get behind the paywall (e.g., proxying from an external network into a campus network), consider deploying the bypass service inside the campus network rather than locally. Then configure your system proxy to forward to the bypass service, and set the bypass service host and port in the app settings. Alternatively, use the Cloudflare Proxy setting in the app (untested).
- On the first request, this project automatically installs CloakBrowser (~200 MB); the initial response will be slow. Additionally, each time the cookie cache expires, the challenges must be re-solved, which will also be slow.

<div align="center">

# Acknowledgements

</div>

- [Pot](https://github.com/pot-app/pot-desktop) — upstream project, providing all core features
- [AnyStyle](https://github.com/inukshuk/anystyle) — citation parsing engine
- [Semantic Scholar](https://www.semanticscholar.org/) — academic metadata API
- [Crossref](https://www.crossref.org/) - academic metadata API
- [Tauri](https://github.com/tauri-apps/tauri) — Rust-powered cross-platform GUI framework
- [Cloudflare Bypass](https://github.com/sarperavci/CloudflareBypassForScraping) — Cloudflare bypass tool

</div>
