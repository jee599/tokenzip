<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  La salida de Claude está llena de ruido innecesario.<br>
  ContextZip la comprime un 60-90%. <code>npx contextzip</code> → 5 segundos (la primera ejecución descarga el binario).
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-instálalo-ahora">Instalar</a> •
  <a href="#-ve-la-diferencia">Before/After</a> •
  <a href="#-los-números-no-mienten">Benchmark</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  Español •
  <a href="README.pt.md">Português</a> •
  <a href="README.hi.md">हिन्दी</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ru.md">Русский</a> •
  <a href="README.tr.md">Türkçe</a> •
  <a href="README.vi.md">Tiếng Việt</a>
</p>

---

```
  30 líneas de stacktrace de node_modules    →    3 líneas
  150 líneas de warnings npm deprecated      →    3 líneas
  50 líneas de hashes de Docker build        →    1 línea
  Colores ANSI, spinners, barras de progreso →    eliminados
```

<h3 align="center">⬇️ Una línea. Eso es todo.</h3>

```bash
npx contextzip
```

<p align="center">Reinicia Claude Code. Cada comando se comprime automáticamente. Sin configuración.<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>Otros métodos de instalación</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Desarrolladores Rust
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 Ve la Diferencia

### 💥 Error Node.js — 30 líneas → 3 líneas (92% ahorrado)

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

### 📦 npm install — 150 líneas → 3 líneas (58-95% ahorrado, varía según cantidad de warnings)

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




Seguridad conservada. Ruido eliminado.

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker Build — 50 líneas → 1 línea (96% ahorrado)

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





Seguridad conservada. Ruido eliminado.


💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — frames del framework ocultos (72% ahorrado)

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

### 🦀 Rust Panic — std/tokio eliminados (2-7% ahorrado vía filtro err)

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


💾 saved 2-7% (filtro err; formato Rust panic aún no optimizado)
```

</td>
</tr>
</table>

### 🔨 TypeScript Build — 40 errores agrupados (81% ahorrado)

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

### 🌐 Página web — nav/footer/anuncios eliminados (73% ahorrado)

<table>
<tr>
<td width="50%">

**❌ Before (salida curl)**
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
enlaces sociales — todo eliminado.
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / Spinners — ruido invisible eliminado (83% ahorrado)

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




Solo se conservan los estados finales.

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Fallo de Docker — contexto preservado</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> Paso fallido + 2 pasos anteriores + mensaje de error + código de salida. Siempre.

</details>

<details>
<summary><b>☕ Java / 🐹 Go stacktraces</b></summary>

**Java** — elimina `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal`:
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

**Go** — elimina `runtime/`, `runtime.gopanic`, `runtime.main`:
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

## 📊 Los Números No Mienten

> **102 pruebas reales. Sin cherry-picking.**

| Categoría | Pruebas | Ahorro Promedio | 🏆 Mejor | 💀 Peor |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker build | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/spinners | 15 | **83%** | 98% | 0% |
| 💥 Error traces | 20 | **59%** | 97% | -12% |
| 🔨 Build errors | 15 | **56%** | 90% | -10% |
| 🌐 Páginas web | 15 | **43%** | 64% | 5% |
| 💻 Comandos CLI | 12 | **42%** | 99% | -56% |
| 📦 Instalación de paquetes | 15 | **39%** | 99% | 2% |

**Total ponderado: 61% de ahorro** → 326K chars in, 127K chars out

> [!NOTE]
> Negativo = la salida creció. Ocurre con entradas muy pequeñas. Mostramos los peores números porque ocultarlos sería deshonesto. [Benchmark completo →](benchmark-results.md)

---

## 🏎️ Cómo Funciona

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

## 🆚 ¿Por Qué No Solo RTK?

Basado en [RTK](https://github.com/rtk-ai/rtk) (28k⭐). Incluye los 34 comandos de RTK. **Además:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| Compresión CLI (git, test, ls) | ✅ | ✅ |
| Stacktraces de errores (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| Extracción de contenido web | ❌ | ✅ |
| Eliminación de ANSI / spinners / decoración | 🟡 | ✅ |
| Agrupación de errores de build (tsc/eslint/cargo) | 🟡 | ✅ |
| Ruido de instalación de paquetes (npm/pip/cargo) | ❌ | ✅ |
| Compresión de Docker build | 🟡 | ✅ |
| Ahorro por comando | ❌ | ✅ |

---

## 📈 Rastrea Todo

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
  <code>--graph</code> gráfico diario &nbsp;•&nbsp; <code>--history</code> comandos recientes
</p>

---

## 🛡️ Nada Importante Se Pierde

| | |
|:---|:---|
| 🔴 Mensajes de error | **SIEMPRE** preservados |
| 📍 Archivo:línea en errores de build | **NUNCA** eliminados |
| 🔒 Alertas de seguridad (CVE, GHSA) | **SIEMPRE** conservadas |
| 🐳 Contexto de fallos Docker | **SIEMPRE** preservado |
| ⏎ Códigos de salida | **SIEMPRE** propagados |

> [!IMPORTANT]
> ContextZip solo elimina **ruido confirmado**. Ante la duda → passthrough.

---

## 🔧 Comandos

```bash
# Automático (el hook los transforma — sin prefijo):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# Manual:
contextzip web https://docs.example.com    # página → solo contenido
contextzip err node server.js              # salida enfocada en errores

# Analítica:
contextzip gain                  # dashboard
contextzip gain --by-feature     # estadísticas por filtro
contextzip gain --graph          # gráfico diario
contextzip gain --history        # comandos recientes

# Gestión:
contextzip init --show           # verificar configuración
contextzip update                # auto-actualización
contextzip uninstall             # desinstalación limpia
```

---

## 🤝 Contribuir

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 Telemetría

ContextZip recopila estadísticas de uso anónimas (cantidad de comandos, porcentaje de ahorro) para mejorar la herramienta. No se transmiten datos personales ni contenido de comandos.

**Desactivar:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# o en ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 Licencia

MIT — Fork de [RTK](https://github.com/rtk-ai/rtk) por rtk-ai.

---

<p align="center">
  <b>⚡ Menos ruido. Más código. Despliega más rápido.</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
