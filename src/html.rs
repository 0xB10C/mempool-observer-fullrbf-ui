use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TransactionContext {
    pub txid: String,
    pub fee: u64,
    pub vsize: u64,
    pub feerate: String,
    pub raw: String,
    pub op_return: bool,
}

#[derive(Serialize, Clone)]
pub struct ReplacementDeltaContext {
    pub fee: i64,
    pub vsize: i64,
    pub feerate: String,
}

#[derive(Serialize, Clone)]
pub struct ReplacementContext {
    pub timestamp: u64,
    pub replaced: TransactionContext,
    pub replacement: TransactionContext,
    pub deltas: ReplacementDeltaContext,
}

#[derive(Serialize)]
pub struct NavigationContext {
    pub pages: Vec<u32>,
}

#[derive(Serialize)]
pub struct SiteContext {
    pub replacements: Vec<ReplacementContext>,
    pub num_replacements: usize,
    pub timestamp: u64,
    pub page: u32,
    pub navigation: NavigationContext,
}

pub static TEMPLATE_TX: &str = r###"
<div class="row">
    <div class="col-12">
        {{ if op_return}}
            <span class="badge text-bg-secondary">OP_RETURN</span>
        {{ endif }}
        <a href="#" class="badge text-bg-light badge-mined text-decoration-none" target="_blank" aria-txid="{txid}">loading..</a>
    </div>
    <div class="col-12 text-truncate">
        {txid}
    </div>
    <div class="col-12">
        {feerate} sat/vByte ({fee}sat, {vsize} vByte)
    </div>
    <details class="col-12">
        <summary>
            raw transaction
        </summary>
        <pre>{raw}</pre>
    </details>
</div>
"###;

pub static TEMPLATE_DELTAS: &str = r#"
<div class="row text-center">
    <div class="col-12">
        <span style="font-size: 6em; line-height:0.4em; color: gray">→</span>
    </div>
    <div class="col-12">
        <span>+{fee} sat</span>
        {{ if vsize }}
            <span>{vsize} vByte</span>
        {{ endif }}
    </div>
    <div class="col-12">
        <span>+{feerate} sat/vByte</span>
    </div>
</div>
"#;

pub static TEMPLATE_REPLACEMENT: &str = r#"
<div class="card-body">
    <div class="row">
        <div class="col-4">
            {{- call tmpl_transaction with replaced -}}
        </div>
        <div class="col-4">
            {{- call tmpl_deltas with deltas -}}
        </div>
        <div class="col-4">
            {{- call tmpl_transaction with replacement -}}
        </div>
    </div>
</div>
<div class="card-footer">
    <div class="col-12">
        <span class="timestamp" aria-timestamp="{timestamp}">timestamp</span>
    </div>
</div>
"#;

pub static TEMPLATE_PAGE_NAVIGATION: &str = r###"
<nav aria-label="Page navigation">
    <ul class="pagination justify-content-center">
        {{ for page in pages }}
            {{ if not page }}
                <li class="page-item">
                    <a href="/">
                        <span class="page-link">{page}</span>
                    </a>
                </li>
            {{ else }}
                <li class="page-item">
                    <a href="page_{page}.html">
                        <span class="page-link">{page}</span>
                    </a>
                </li>
            {{ endif }}
        {{ endfor }}
    </ul>
</nav>
"###;

pub static TEMPLATE_SITE: &str = r###"
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="Showing recent full-RBF replacements">
    <meta name="author" content="0xB10C">
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous">
    <title>Recent full-RBF replacements {{ if page }}(page {page}){{ endif }} - mempool.observer</title>
  </head>
  <body>

  <header>
    <nav class="navbar border-bottom mb-5">
        <div class="container">
            <span class="d-inline-block navbar-brand">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 143.05 37.854" height="70" width="300">
                    <g class="text" aria-label="mem" style="line-height:.01%" fill="#1a1a1a" font-family="Laksaman" font-size="25.4" font-weight="400" letter-spacing="0" word-spacing="0">
                    <path d="M14.656 5.991q2.26 0 3.429 1.702 1.168 1.702 1.168 4.343v7.214H17.73v-7.188q0-2.134-.863-3.455-.839-1.346-2.617-1.346-1.447 0-2.59 1.143-1.143 1.143-1.143 2.591v8.255H8.992v-7.874q0-4.115-3.277-4.115-1.499 0-2.718 1.296-1.219 1.295-1.219 2.819v7.874H.254V9.471q0-1.143-.254-3.2h1.397q.254.787.305 2.311 1.473-2.59 4.369-2.59 2.641 0 3.937 2.895 1.524-2.896 4.648-2.896zM23.807 12.875q.077 2.463 1.474 3.962 1.422 1.499 3.784 1.499 2.21 0 4.547-.915l.127 1.245q-2.26.94-4.826.94-2.921 0-4.775-1.804-1.855-1.829-1.855-4.699 0-3.073 1.778-5.08 1.804-2.032 4.7-2.032 5.46 0 5.587 6.884zm9.017-1.27q-.05-1.982-1.219-3.15-1.143-1.194-3.022-1.194-1.88 0-3.15 1.245-1.27 1.219-1.55 3.099zM51.912 5.991q2.261 0 3.43 1.702 1.168 1.702 1.168 4.343v7.214h-1.524v-7.188q0-2.134-.864-3.455-.838-1.346-2.616-1.346-1.448 0-2.59 1.143-1.144 1.143-1.144 2.591v8.255h-1.524v-7.874q0-4.115-3.276-4.115-1.499 0-2.718 1.296-1.22 1.295-1.22 2.819v7.874h-1.523V9.471q0-1.143-.254-3.2h1.397q.254.787.304 2.311 1.474-2.59 4.37-2.59 2.64 0 3.936 2.895 1.524-2.896 4.648-2.896z" style="line-height:1.25;-inkscape-font-specification:Laksaman"></path>
                    </g>
                    <g class="text" style="line-height:.01%">
                    <path d="M56.985 30.177q0 2.896-1.854 4.852-1.829 1.955-4.699 1.955-2.896 0-4.724-1.93-1.83-1.956-1.83-4.877 0-2.895 1.855-4.851 1.854-1.956 4.699-1.956 2.87 0 4.699 1.956 1.854 1.956 1.854 4.851zm-6.553 5.537q2.21 0 3.607-1.6 1.422-1.6 1.422-3.937 0-2.337-1.397-3.937-1.397-1.6-3.632-1.6-2.21 0-3.632 1.6-1.397 1.6-1.397 3.937 0 2.337 1.397 3.937 1.422 1.6 3.632 1.6z" style="line-height:1.25;-inkscape-font-specification:Laksaman" fill="#1a1a1a" aria-label="o" font-family="Laksaman" font-size="25.4" font-weight="400" letter-spacing="0" word-spacing="0"></path>
                    </g>
                    <g fill="#c83737" stroke-width="1.832">
                    <path fill-rule="evenodd" d="M58.854 37.627c.407-13.135.434-19.647 0-32.156-.017-.51 1.457 0 1.457 0v32.156s-1.473.51-1.457 0z"></path>
                    <g aria-label="oo" style="line-height:.01%" font-family="sans-serif" font-weight="400" letter-spacing="0" word-spacing="0">
                        <path d="M67.158 22.76q3.115 0 5.218 2.097 2.104 2.068 2.104 5.314 0 3.274-2.076 5.342-2.104 2.068-5.246 2.068-3.114 0-5.218-2.097t-2.104-5.313q0-3.246 2.104-5.314 2.104-2.097 5.218-2.097zm5.956 7.41q0-2.498-1.721-4.078-1.721-1.608-4.235-1.608-2.513 0-4.234 1.58-1.722 1.58-1.722 4.107 0 2.498 1.722 4.107 1.72 1.58 4.234 1.58 2.514 0 4.235-1.58 1.72-1.609 1.72-4.107zM67.158 5.595q3.115 0 5.218 2.096 2.104 2.068 2.104 5.314 0 3.274-2.076 5.342-2.104 2.068-5.246 2.068-3.114 0-5.218-2.097-2.104-2.096-2.104-5.313 0-3.246 2.104-5.314 2.104-2.096 5.218-2.096zm5.956 7.41q0-2.499-1.721-4.079-1.721-1.608-4.235-1.608-2.513 0-4.234 1.58-1.722 1.58-1.722 4.107 0 2.499 1.722 4.107 1.72 1.58 4.234 1.58 2.514 0 4.235-1.58 1.72-1.608 1.72-4.107z"></path>
                    </g>
                    <path d="M64.251 17.276c.818.446 1.976.798 2.937.8 2.875 0 5.33-2.533 5.33-5.077-.024-1.094-.132-1.441-.669-2.147.146 1.93-1.301 4.327-3.323 5.52-1.287.756-2.769.669-4.275.904zM64.251 34.453c.818.446 1.976.797 2.937.8 2.875 0 5.33-2.533 5.33-5.077-.024-1.095-.132-1.442-.669-2.147.146 1.929-1.301 4.327-3.323 5.519-1.287.756-2.769.67-4.275.905z"></path>
                    <path fill-rule="evenodd" d="M62.096 24.735c-.333-2.914-.345-3.415 0-6.155.058-.456 1.363-.458 1.315 0-.33 3.1-.337 3.074 0 6.14.05.458-1.003.339-1.315.015z"></path>
                    </g>
                    <g class="text" aria-label="ool" style="line-height:.01%" fill="#1a1a1a" font-family="Laksaman" font-size="25.4" font-weight="400" letter-spacing="0" word-spacing="0">
                    <path d="M89.22 12.852q0 2.896-1.854 4.852-1.829 1.956-4.7 1.956-2.895 0-4.723-1.93-1.83-1.957-1.83-4.878 0-2.895 1.855-4.851 1.854-1.956 4.699-1.956 2.87 0 4.699 1.956 1.854 1.956 1.854 4.851zm-6.553 5.538q2.21 0 3.607-1.6 1.422-1.6 1.422-3.938 0-2.336-1.397-3.937-1.397-1.6-3.632-1.6-2.21 0-3.632 1.6-1.397 1.6-1.397 3.937 0 2.337 1.397 3.937 1.422 1.6 3.632 1.6zM104.4 12.852q0 2.896-1.854 4.852-1.828 1.956-4.699 1.956-2.895 0-4.724-1.93-1.829-1.957-1.829-4.878 0-2.895 1.854-4.851 1.855-1.956 4.7-1.956 2.87 0 4.698 1.956 1.855 1.956 1.855 4.851zm-6.553 5.538q2.21 0 3.607-1.6 1.423-1.6 1.423-3.938 0-2.336-1.397-3.937-1.397-1.6-3.633-1.6-2.21 0-3.632 1.6-1.397 1.6-1.397 3.937 0 2.337 1.397 3.937 1.423 1.6 3.632 1.6zM112.393 18.39l-.153 1.27q-4.826-.229-4.826-6.884V0h1.524v12.243q0 1.448.127 2.438.127.99.458 1.905.355.915 1.066 1.372.712.432 1.804.432z" style="line-height:1.25;-inkscape-font-specification:Laksaman"></path>
                    </g>
                    <g class="text" aria-label="server" style="line-height:.01%" fill="#1a1a1a" font-family="Laksaman" font-size="25.4" font-weight="400" letter-spacing="0" word-spacing="0">
                    <path d="M80.09 24.28q-1.067 0-1.778.61-.71.583-.71 1.549 0 .71.736 1.397.762.685 2.133 1.168 3.48 1.194 3.48 3.835 0 1.778-1.346 2.794-1.32.991-3.353.991-1.88 0-3.658-1.041l.56-1.22q1.828.991 3.225.991 3.048 0 3.048-2.413 0-.889-.737-1.6-.71-.737-2.133-1.169-3.48-1.066-3.48-3.53 0-1.651 1.22-2.642 1.244-.99 3.073-.99 2.006 0 3.2.889l-.61 1.295q-1.143-.914-2.87-.914zM87.808 29.893q.076 2.464 1.473 3.962 1.422 1.499 3.784 1.499 2.21 0 4.547-.914l.127 1.244q-2.26.94-4.826.94-2.921 0-4.775-1.803-1.854-1.83-1.854-4.7 0-3.073 1.778-5.08 1.803-2.031 4.699-2.031 5.46 0 5.588 6.883zm9.017-1.27q-.051-1.981-1.22-3.15-1.143-1.193-3.022-1.193-1.88 0-3.15 1.244-1.27 1.22-1.55 3.099zM107.48 24.33q-.153-.05-.457-.05-1.677 0-2.845 1.371-1.143 1.346-1.143 3.632v6.985h-1.524v-8.737q0-2.87-.381-4.242h1.295q.407.813.407 2.59 1.219-2.87 3.911-2.87.356 0 .737.077zM120.718 23.289l-5.842 12.98h-1.321l-5.614-12.98h1.524l4.801 11.38 4.928-11.38zM123.377 29.893q.077 2.464 1.474 3.962 1.422 1.499 3.784 1.499 2.21 0 4.547-.914l.127 1.244q-2.26.94-4.826.94-2.921 0-4.775-1.803-1.855-1.83-1.855-4.7 0-3.073 1.778-5.08 1.804-2.031 4.7-2.031 5.46 0 5.587 6.883zm9.017-1.27q-.05-1.981-1.219-3.15-1.143-1.193-3.022-1.193-1.88 0-3.15 1.244-1.27 1.22-1.55 3.099zM143.05 24.33q-.153-.05-.457-.05-1.677 0-2.845 1.371-1.143 1.346-1.143 3.632v6.985h-1.524v-8.737q0-2.87-.381-4.242h1.295q.407.813.407 2.59 1.219-2.87 3.911-2.87.356 0 .737.077z" style="line-height:1.25;-inkscape-font-specification:Laksaman"></path>
                    </g>
                </svg>
            </span>
        </div>
    </nav>
  </header>

  <main class="container">
    <h1>Recent full-RBF replacements {{if page }}(page {page}){{ endif }}</h1>
    <p>
        Showing {num_replacements} of the most recent full-RBF replacement events my <code>mempoolfullrbf=1</code> node saw. I assume that a replacement is a full-RBF replacement, if the replaced transaction does not signal BIP-125 replaceability.
    </p>

    <p>
        Transactions that confirmed in a block (queried from the blockstream.info API) are labeled as <span class="badge text-bg-warning">mined in X</span>.
    </p>

    {{- call tmpl_navigation with navigation -}}

    <div class="row px-3">
        <div class="col-4">
            <h2>replaced</h2>
        </div>
        <div class="col-4"></div>
        <div class="col-4">
            <h2>replacement</h2>
        </div>
    </div>

    {{ for replacement in replacements }}
        <div class="card m-3">
            {{- call tmpl_replacement with replacement -}}
        </div>
    {{ endfor }}

    {{- call tmpl_navigation with navigation -}}

  </main>
  <footer class="container text-muted border-top">
    <p>
        by <a href="https://b10c.me">0xb10c</a> | site generated at <span class="timestamp" aria-timestamp="{timestamp}">timestamp</span> with <a href="https://github.com/0xB10C/mempool-observer-fullrbf-ui">github.com/0xB10C/mempool-observer-fullrbf-ui</a>
    </p>
    </footer>

<script>

    const minedBadges = document.getElementsByClassName("badge-mined");
    for(const badge of minedBadges) \{
        fetch("https://blockstream.info/api/tx/" + badge.getAttribute('aria-txid'))
        .then((response) => \{
            if (response.status === 404) \{
                badge.remove()
            } else if (response.status === 200 ) \{
                return response.json()
            }
        }).catch((error) => \{
            console.error(error);
        }).then(response => \{
                if (response) \{
                    if (response.status.confirmed) \{
                        badge.classList.add('text-bg-warning');
                        badge.classList.remove('text-bg-light');
                        badge.innerHTML = "mined in " + response.status.block_height;
                        badge.setAttribute("href", "https://miningpool.observer/template-and-block/"+response.status.block_hash)
                    } else \{
                        badge.innerHTML = "in blockstream.info mempool";
                    }
                }
                console.log(response)
            }
        );
    }

    const timestamps = document.getElementsByClassName("timestamp");
    for(const timestampSpan of timestamps) \{
        let date = new Date(timestampSpan.getAttribute('aria-timestamp')*1000);
        timestampSpan.innerHTML = date.toLocaleTimeString() + " on " + date.toLocaleDateString()
    }

</script>

</body>
</html>
"###;
