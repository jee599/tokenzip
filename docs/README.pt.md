<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  A saída do Claude é cheia de ruído desnecessário.<br>
  ContextZip comprime 60-90%. <code>npx contextzip</code> → 5 segundos (primeira execução baixa o binário).
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-instale-agora">Instalar</a> •
  <a href="#-veja-a-diferença">Before/After</a> •
  <a href="#-os-números-não-mentem">Benchmark</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.es.md">Español</a> •
  Português •
  <a href="README.hi.md">हिन्दी</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ru.md">Русский</a> •
  <a href="README.tr.md">Türkçe</a> •
  <a href="README.vi.md">Tiếng Việt</a>
</p>

---

```
  30 linhas de stacktrace node_modules       →    3 linhas
  150 linhas de warnings npm deprecated      →    3 linhas
  50 linhas de hashes de Docker build        →    1 linha
  Cores ANSI, spinners, barras de progresso  →    removidos
```

<h3 align="center">⬇️ Uma linha. Só isso.</h3>

```bash
npx contextzip
```

<p align="center">Reinicie o Claude Code. Todo comando é comprimido automaticamente. Zero configuração.<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>Outros métodos de instalação</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Desenvolvedores Rust
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 Veja a Diferença

### 💥 Erro Node.js — 30 linhas → 3 linhas (92% economizado)

<table>
<tr>
<td width="50%">

**❌ Before**
```
TypeError: Cannot read properties
  of undefined (reading 'id')
    at getUserProfile (users.ts:47)
    at processAuth (auth.ts:12)
    at Layer.handle (node_modules/
      express/lib/router/layer.js:95)
    at next (node_modules/express/
      lib/router/route.js:144)
    at Route.dispatch (node_modules/
      express/lib/router/route.js:114)
    ... 25 more node_modules lines
```

</td>
<td width="50%">

**✅ After**
```
TypeError: Cannot read properties
  of undefined (reading 'id')
  → users.ts:47    getUserProfile()
  → auth.ts:12     processAuth()
  (+ 27 framework frames hidden)



💾 saved 92%
```

</td>
</tr>
</table>

### 📦 npm install — 150 linhas → 3 linhas (58-95% economizado, varia por quantidade de warnings)

<table>
<tr>
<td width="50%">

**❌ Before**
```
npm warn deprecated inflight@1.0.6
npm warn deprecated rimraf@3.0.2
npm warn deprecated glob@7.2.3
npm warn deprecated bcrypt@3.0.0:
  security vulnerability CVE-2023-31484
... 45 more deprecated warnings
added 847 packages, audited 848
143 packages looking for funding
  run `npm fund` for details
8 vulnerabilities (2 moderate, 6 high)
  To address issues: npm audit fix
  ... 20 more lines
```

</td>
<td width="50%">

**✅ After**
```
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 mod)
⚠ bcrypt@3.0.0: CVE-2023-31484




Segurança mantida. Ruído removido.

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker Build — 50 linhas → 1 linha (96% economizado)

<table>
<tr>
<td width="50%">

**❌ Before**
```
Step 1/12 : FROM node:20-alpine
 ---> abc123def456
Step 2/12 : WORKDIR /app
 ---> Using cache
 ---> 789ghi012jkl
Step 3/12 : COPY package*.json ./
 ---> Using cache
... 8 more steps with hashes
Removing intermediate container xyz
Successfully built abc123final
Successfully tagged my-app:latest
```

</td>
<td width="50%">

**✅ After**
```
✓ built my-app:latest (12 steps, 8 cached)






Segurança mantida. Ruído removido.

💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — frames do framework ocultos (72% economizado)

<table>
<tr>
<td width="50%">

**❌ Before**
```
Traceback (most recent call last):
  File "/app/main.py", line 10,
    in handler
    process(data)
  File "/usr/lib/python3.11/
    importlib/__init__.py", line 126
  File "/app/venv/lib/site-packages/
    flask/app.py", line 1498
  File "/app/venv/lib/site-packages/
    flask/app.py", line 1476
ValueError: invalid literal for int()
```

</td>
<td width="50%">

**✅ After**
```
Traceback (most recent call last):
  → /app/main.py:10  process(data)
  (+ 3 framework frames hidden)
ValueError: invalid literal for int()




💾 saved 72%
```

</td>
</tr>
</table>

### 🦀 Rust Panic — std/tokio removidos (2-7% economizado via filtro err)

<table>
<tr>
<td width="50%">

**❌ Before**
```
thread 'main' panicked at
  'index out of bounds',
  src/handler.rs:42:5
stack backtrace:
   0: std::panicking::begin_panic
   1: core::panicking::panic_fmt
   2: myapp::handler::process
        at ./src/handler.rs:42:5
   3: myapp::main
        at ./src/main.rs:15:3
   4: std::rt::lang_start
   5: tokio::runtime::enter
```

</td>
<td width="50%">

**✅ After**
```
thread 'main' panicked at
  'index out of bounds',
  src/handler.rs:42:5
  (+ 2 framework frames hidden)
  → handler.rs:42  process()
  → main.rs:15     main()
  (+ 2 framework frames hidden)


💾 saved 2-7% (filtro err; formato Rust panic ainda não otimizado)
```

</td>
</tr>
</table>

### 🔨 TypeScript Build — 40 erros agrupados (81% economizado)

<table>
<tr>
<td width="50%">

**❌ Before**
```
src/api/users.ts:47:5 - error TS2322:
  Type 'string' not assignable to 'number'
src/api/users.ts:83:5 - error TS2322:
  Type 'string' not assignable to 'number'
src/api/orders.ts:12:5 - error TS2322:
  Type 'string' not assignable to 'number'
src/api/orders.ts:45:5 - error TS2322:
  Type 'string' not assignable to 'number'
... 36 more identical errors
Found 40 errors in 8 files.
```

</td>
<td width="50%">

**✅ After**
```
TS2322: Type 'string' not assignable
        to type 'number' (×40)
  src/api/users.ts    :47, :83
  src/api/orders.ts   :12, :45, :67
  src/api/products.ts :23, :89
  src/lib/helpers.ts  :156
  ... +4 files (28 occurrences)

All line numbers preserved.
💾 saved 81%
```

</td>
</tr>
</table>

### 🌐 Página web — nav/footer/anúncios removidos (73% economizado)

<table>
<tr>
<td width="50%">

**❌ Before (saída curl)**
```
[Skip to content]
[Nav: Products, Pricing, Docs, Blog]
[Sidebar: Getting Started, Auth,
  Database, Storage, Functions]
# Email/Password Authentication
Use supabase.auth.signInWithPassword
  to sign in users...
[code example]
[code example]
[Footer: © 2026 Supabase Inc]
[Terms | Privacy | Status]
[Newsletter: Subscribe for updates]
[Social: Twitter GitHub Discord]
```

</td>
<td width="50%">

**✅ After**
```
# Email/Password Authentication
Use supabase.auth.signInWithPassword
  to sign in users...
[code example]
[code example]




Nav, footer, sidebar, newsletter,
links sociais — tudo removido.
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / Spinners — ruído invisível removido (83% economizado)

<table>
<tr>
<td width="50%">

**❌ Before (raw terminal)**
```
\033[32m✓ Success\033[0m
\033[31m✗ Error\033[0m
⠋ Installing dependencies...
⠙ Installing dependencies...
⠹ Installing dependencies...
⠸ Installing dependencies...
████░░░░░░ 40%
████████░░ 80%
██████████ 100%
═══════════════════════
Done.
```

</td>
<td width="50%">

**✅ After**
```
✓ Success
✗ Error
██████████ 100%
Done.




Apenas estados finais mantidos.

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Falha Docker — contexto preservado</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> Passo que falhou + 2 passos anteriores + mensagem de erro + código de saída. Sempre.

</details>

<details>
<summary><b>☕ Java / 🐹 Go stacktraces</b></summary>

**Java** — remove `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal`:
```diff
- java.lang.NullPointerException: Cannot invoke method on null
-   at com.myapp.UserService.getUser(UserService.java:42)
-   at com.myapp.Controller.handle(Controller.java:15)
-   at java.lang.reflect.Method.invoke(Method.java:498)
-   at sun.reflect.DelegatingMethodAccessorImpl.invoke(...)
-   at org.springframework.web.servlet.FrameworkServlet.service(...)
-   at org.apache.catalina.core.ApplicationFilterChain.internalDoFilter(...)

+ java.lang.NullPointerException: Cannot invoke method on null
+   at com.myapp.UserService.getUser(UserService.java:42)
+   at com.myapp.Controller.handle(Controller.java:15)
+   (+ 4 framework frames hidden)
```

**Go** — remove `runtime/`, `runtime.gopanic`, `runtime.main`:
```diff
- goroutine 1 [running]:
- runtime/debug.Stack()
-   /usr/local/go/src/runtime/debug/stack.go:24
- runtime.gopanic({0x1234, 0x5678})
-   /usr/local/go/src/runtime/panic.go:884
- main.handler()
-   /app/handler.go:42 +0x1a4
- main.main()
-   /app/main.go:15 +0x58

+ goroutine 1 [running]:
+   (+ 2 framework frames hidden)
+   → main.handler()  /app/handler.go:42
+   → main.main()     /app/main.go:15
+   (+ 1 framework frames hidden)
```

</details>

---

## 📊 Os Números Não Mentem

> **102 testes reais. Sem cherry-picking.**

| Categoria | Testes | Economia Média | 🏆 Melhor | 💀 Pior |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker build | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/spinners | 15 | **83%** | 98% | 0% |
| 💥 Error traces | 20 | **59%** | 97% | -12% |
| 🔨 Build errors | 15 | **56%** | 90% | -10% |
| 🌐 Páginas web | 15 | **43%** | 64% | 5% |
| 💻 Comandos CLI | 12 | **42%** | 99% | -56% |
| 📦 Instalação de pacotes | 15 | **39%** | 99% | 2% |

**Total ponderado: 61% de economia** → 326K chars in, 127K chars out

> [!NOTE]
> Negativo = saída cresceu. Acontece com entradas muito pequenas. Colocamos os piores números na tabela porque escondê-los seria desonesto. [Benchmark completo →](benchmark-results.md)

---

## 🏎️ Como Funciona

```
  ┌─────────────────────────────────────────────┐
  │  Claude Code runs: git status               │
  │                         ↓                   │
  │  Hook rewrites → contextzip git status      │
  │                         ↓                   │
  │  ┌──────────────────────────────────────┐   │
  │  │ [1] ANSI preprocessor    strip junk  │   │
  │  │ [2] Command router    40+ filters    │   │
  │  │ [3] Error post-proc   compress stack │   │
  │  │ [4] SQLite tracker    record savings │   │
  │  └──────────────────────────────────────┘   │
  │                         ↓                   │
  │  Compressed output → Claude's context       │
  │  💾 contextzip: 200 → 40 tokens (80%)       │
  └─────────────────────────────────────────────┘
```

---

## 🆚 Por Que Não Só RTK?

Baseado no [RTK](https://github.com/rtk-ai/rtk) (28k⭐). Inclui todos os 34 comandos do RTK. **Além disso:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| Compressão CLI (git, test, ls) | ✅ | ✅ |
| Stacktraces de erros (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| Extração de conteúdo web | ❌ | ✅ |
| Remoção de ANSI / spinners / decoração | 🟡 | ✅ |
| Agrupamento de erros de build (tsc/eslint/cargo) | 🟡 | ✅ |
| Ruído de instalação de pacotes (npm/pip/cargo) | ❌ | ✅ |
| Compressão de Docker build | 🟡 | ✅ |
| Economia por comando | ❌ | ✅ |

---

## 📈 Rastreie Tudo

```bash
$ contextzip gain
📊 ContextZip Token Savings
════════════════════════════════════════
Total commands:    2,927
Tokens saved:      10.3M (89.2%)
Efficiency meter: █████████████████████░░░ 89%

$ contextzip gain --by-feature
Feature        Commands  Saved     Avg%
cli (RTK)      2,100     6.8M     78%
error          89        1.2M     93%
web            43        0.9M     73%
build          112       0.4M     81%
pkg            34        0.3M     95%
docker         22        0.2M     85%
```

<p align="center">
  <code>--graph</code> gráfico diário &nbsp;•&nbsp; <code>--history</code> comandos recentes
</p>

---

## 🛡️ Nada Importante Se Perde

| | |
|:---|:---|
| 🔴 Mensagens de erro | **SEMPRE** preservadas |
| 📍 Arquivo:linha em erros de build | **NUNCA** removidos |
| 🔒 Alertas de segurança (CVE, GHSA) | **SEMPRE** mantidos |
| 🐳 Contexto de falhas Docker | **SEMPRE** preservado |
| ⏎ Códigos de saída | **SEMPRE** propagados |

> [!IMPORTANT]
> ContextZip só remove **ruído confirmado**. Na dúvida → passthrough.

---

## 🔧 Comandos

```bash
# Automático (o hook transforma — sem prefixo):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# Manual:
contextzip web https://docs.example.com    # página → só conteúdo
contextzip err node server.js              # saída focada em erros

# Análise:
contextzip gain                  # dashboard
contextzip gain --by-feature     # estatísticas por filtro
contextzip gain --graph          # gráfico diário
contextzip gain --history        # comandos recentes

# Gerenciamento:
contextzip init --show           # verificar configuração
contextzip update                # auto-atualização
contextzip uninstall             # desinstalação limpa
```

---

## 🤝 Contribuir

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 Telemetria

ContextZip coleta estatísticas de uso anônimas (contagem de comandos, percentual de economia) para melhorar a ferramenta. Nenhum dado pessoal ou conteúdo de comandos é transmitido.

**Desativar:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# ou em ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 Licença

MIT — Fork de [RTK](https://github.com/rtk-ai/rtk) por rtk-ai.

---

<p align="center">
  <b>⚡ Menos ruído. Mais código. Deploy mais rápido.</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
