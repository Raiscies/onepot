## Publisher Handlers

| Publisher | Domain | Bypass | Method | Status |
|--------|------|------------|------|------|
| ACM | `dl.acm.org` | Cloudflare | `/doi/pdf/{doi}` | ✅ |
| Springer | `link.springer.com` | Cloudflare | `/content/pdf/{doi}.pdf` | ✅ |
| SIAM | `epubs.siam.org` | Cloudflare | `/doi/pdf/{doi}?download=true` | ✅ |
| APS | `journals.aps.org` | Cloudflare |`/pre/pdf/{doi}` | ✅ |
| Dagstuhl | `drops.dagstuhl.de` | None | scrapes from page | ✅ |
| IEEE | `ieeexplore.ieee.org` | None | `/stampPDF/getPDF.jsp?arnumber={id}` | ✅ |
| Nature | `www.nature.com` | None | `/articles/{id}.pdf` | ✅ |
| Science | `www.science.org` | Cloudflare | `/doi/pdf/{doi}?download=true` | ✅ | 
| ArXiv | `arxiv.org` | None | `/pdf/{id}` | ✅ |
| Wiley | `*.onlinelibrary.wiley.com` | Cloudflare | `/doi/pdfdirect/{doi}` | ✅ |
| Elsevier | `linkinghub.elsevier.com` | Cloudflare | scrapes from page, solves JS challenge | ❌ |
| Sage | `journals.sagepub.com` | Cloudflare | tokenized, needs headless | ❌ |
| IOP | `iopscience.iop.org` | reCAPTCHA | `/article/{doi}/pdf` | ❌ |

### DOI / Links For Tests

- **link.springer.com**: `10.1007/978-3-540-68552-4_24` ✅ \
    -> `https://link.springer.com/chapter/10.1007/978-3-540-68552-4_24` \
    -> `https://link.springer.com/content/pdf/10.1007/978-3-540-68552-4.pdf`

- **drops.dagstuhl.de**: `10.4230/LIPIcs.CPM.2021.15` ✅ \
    -> Scrape: `a.fixed-pdf-button[href][title=\"View as PDF\"]@href` \
    old regex: `href=\"(https://drops\\.dagstuhl\\.de/storage/[^\"]+\\.pdf)\"`

- **www.nature.com**: `10.1038/s41467-018-04978-z` ✅

- **www.science.org**: `10.1126/science.aeg8744` ✅

- **epubs.siam.org**: `10.1137/0136016` ✅ \
    -> `https://epubs.siam.org/doi/10.1137/0136016` \
    -> `https://epubs.siam.org/doi/epdf/10.1137/0136016` \
    -> `https://epubs.siam.org/doi/pdf/10.1137/0136016`

- **arxiv.org**: `10.48550/arXiv.2207.03579` ✅ \
    -> `https://arxiv.org/abs/2207.03579` \
    -> `https://arxiv.org/pdf/2207.03579`

- **journals.aps.org**: `10.1103/PHYSREVE.76.056709` ✅ \
    -> `https://journals.aps.org/pre/abstract/10.1103/PhysRevE.76.056709` \
    -> `https://journals.aps.org/pre/pdf/10.1103/PhysRevE.76.056709`

- **\*.onlinelibrary.wiley.com**: `10.1016/j.febslet.2009.12.039` \
    -> `https://febs.onlinelibrary.wiley.com/doi/10.1016/j.febslet.2009.12.039` \
    -> `https://febs.onlinelibrary.wiley.com/doi/pdfdirect/10.1016/j.febslet.2009.12.039`

**Difficult:**

- **journals.sagepub.com**: `10.1068/b306` — tokenized, needs headless browser
- **iopscience.iop.org**: `10.1088/1755-1315/526/1/012190` — reCAPTCHA redirect \
    -> `https://iopscience.iop.org/article/10.1088/1755-1315/526/1/012190` \
    -> `https://iopscience.iop.org/article/10.1088/1755-1315/526/1/012190/pdf`

- **linkinghub.elsevier.com**: `10.1016/J.PHYSA.2014.05.073` \
    -> `https://linkinghub.elsevier.com/retrieve/pii/S152659002400350X` \
    -> `https://www.sciencedirect.com/science/article/pii/S152659002400350X?via%3Dihub` \
    -> Scrape: `a.link-button[href]@href` \
    -> `https://www.sciencedirect.com/science/article/pii/S152659002400350X/pdfft?md5=4817c30a30cd40d59ef66331ade7463b&pid=1-s2.0-S152659002400350X-main.pdf` \
    -> Solves JS Challenge, and follow redirect \
    -> `https://pdf.sciencedirectassets.com/272520/1-s2.0-S1526590023X00170/1-s2.0-S152659002400350X/main.pdf?X-Amz-Security-Token=...(VERY LONG)`
    
## TODO

- [ ] CF bypass server support headed mode + captcha checks，pop up explorer window and inform users to verify manually
- [ ] Sage headless browser handler
