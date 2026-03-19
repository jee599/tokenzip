<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Claude का आउटपुट अनावश्यक शोर से भरा है।<br>
  ContextZip इसे 60-90% तक कम करता है। <code>npx contextzip</code> → 5 सेकंड (पहली बार बाइनरी डाउनलोड होती है)।
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-अभी-इंस्टॉल-करें">इंस्टॉल</a> •
  <a href="#-अंतर-देखें">Before/After</a> •
  <a href="#-नंबर-झूठ-नहीं-बोलते">बेंचमार्क</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.pt.md">Português</a> •
  हिन्दी •
  <a href="README.fr.md">Français</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ru.md">Русский</a> •
  <a href="README.tr.md">Türkçe</a> •
  <a href="README.vi.md">Tiếng Việt</a>
</p>

---

```
  30 लाइनें node_modules स्टैकट्रेस       →    3 लाइनें
  150 लाइनें npm deprecated चेतावनियाँ     →    3 लाइनें
  50 लाइनें Docker बिल्ड हैश              →    1 लाइन
  ANSI कलर, स्पिनर, प्रोग्रेस बार        →    हटा दिए
```

<h3 align="center">⬇️ एक लाइन। बस इतना ही।</h3>

```bash
npx contextzip
```

<p align="center">Claude Code रीस्टार्ट करें। हर कमांड अपने आप कंप्रेस होता है। कोई कॉन्फ़िगरेशन नहीं।<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>अन्य इंस्टॉलेशन विधियाँ</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Rust डेवलपर्स
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 अंतर देखें

### 💥 Node.js एरर — 30 लाइनें → 3 लाइनें (92% बचत)

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

### 📦 npm install — 150 लाइनें → 3 लाइनें (58-95% बचत, चेतावनियों की संख्या के अनुसार)

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




सुरक्षा जानकारी सुरक्षित। शोर हटाया गया।

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker बिल्ड — 50 लाइनें → 1 लाइन (96% बचत)

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






सुरक्षा जानकारी सुरक्षित। शोर हटाया गया।

💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — फ्रेमवर्क फ़्रेम छुपाए गए (72% बचत)

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

### 🦀 Rust Panic — std/tokio हटाए गए (err फ़िल्टर से 2-7% बचत)

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


💾 saved 2-7% (err फ़िल्टर; Rust panic फ़ॉर्मेट अभी ऑप्टिमाइज़्ड नहीं)
```

</td>
</tr>
</table>

### 🔨 TypeScript बिल्ड — 40 एरर ग्रुप किए गए (81% बचत)

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

### 🌐 वेब पेज — nav/footer/विज्ञापन हटाए गए (73% बचत)

<table>
<tr>
<td width="50%">

**❌ Before (curl आउटपुट)**
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
सोशल लिंक — सब हटा दिए।
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / स्पिनर — अदृश्य शोर हटाया गया (83% बचत)

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




केवल अंतिम स्थितियाँ रखी गईं।

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Docker विफलता — संदर्भ संरक्षित</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> विफल चरण + पिछले 2 चरण + एरर मैसेज + एग्ज़िट कोड। हमेशा।

</details>

<details>
<summary><b>☕ Java / 🐹 Go स्टैकट्रेस</b></summary>

**Java** — `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal` हटाता है:
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

**Go** — `runtime/`, `runtime.gopanic`, `runtime.main` हटाता है:
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

## 📊 नंबर झूठ नहीं बोलते

> **102 वास्तविक परीक्षण। कोई चेरी-पिकिंग नहीं।**

| श्रेणी | परीक्षण | औसत बचत | 🏆 सर्वश्रेष्ठ | 💀 सबसे खराब |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker बिल्ड | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/स्पिनर | 15 | **83%** | 98% | 0% |
| 💥 एरर ट्रेस | 20 | **59%** | 97% | -12% |
| 🔨 बिल्ड एरर | 15 | **56%** | 90% | -10% |
| 🌐 वेब पेज | 15 | **43%** | 64% | 5% |
| 💻 CLI कमांड | 12 | **42%** | 99% | -56% |
| 📦 पैकेज इंस्टॉल | 15 | **39%** | 99% | 2% |

**भारित कुल: 61% बचत** → 326K chars in, 127K chars out

> [!NOTE]
> नकारात्मक = आउटपुट बढ़ गया। बहुत छोटे इनपुट पर ऐसा होता है। सबसे खराब नंबर भी दिखाते हैं क्योंकि छुपाना ईमानदारी नहीं होगी। [पूरा बेंचमार्क →](benchmark-results.md)

---

## 🏎️ यह कैसे काम करता है

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

## 🆚 सिर्फ RTK क्यों नहीं?

[RTK](https://github.com/rtk-ai/rtk) (28k⭐) पर आधारित। RTK के सभी 34 कमांड शामिल। **इसके अलावा:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| CLI कंप्रेशन (git, test, ls) | ✅ | ✅ |
| एरर स्टैकट्रेस (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| वेब पेज कंटेंट एक्सट्रैक्शन | ❌ | ✅ |
| ANSI / स्पिनर / डेकोरेशन हटाना | 🟡 | ✅ |
| बिल्ड एरर ग्रुपिंग (tsc/eslint/cargo) | 🟡 | ✅ |
| पैकेज इंस्टॉल शोर (npm/pip/cargo) | ❌ | ✅ |
| Docker बिल्ड कंप्रेशन | 🟡 | ✅ |
| प्रति-कमांड बचत प्रदर्शन | ❌ | ✅ |

---

## 📈 सब कुछ ट्रैक करें

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
  <code>--graph</code> दैनिक चार्ट &nbsp;•&nbsp; <code>--history</code> हाल के कमांड
</p>

---

## 🛡️ कुछ भी ज़रूरी नहीं खोता

| | |
|:---|:---|
| 🔴 एरर मैसेज | **हमेशा** सुरक्षित |
| 📍 बिल्ड एरर में फ़ाइल:लाइन | **कभी नहीं** हटाए जाते |
| 🔒 सुरक्षा चेतावनियाँ (CVE, GHSA) | **हमेशा** रखी जाती हैं |
| 🐳 Docker विफलता का संदर्भ | **हमेशा** सुरक्षित |
| ⏎ एग्ज़िट कोड | **हमेशा** प्रसारित |

> [!IMPORTANT]
> ContextZip केवल **पुष्ट शोर** हटाता है। संदेह होने पर → passthrough।

---

## 🔧 कमांड

```bash
# स्वचालित (हुक ट्रांसफ़ॉर्म करता है — प्रीफ़िक्स की ज़रूरत नहीं):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# मैनुअल:
contextzip web https://docs.example.com    # पेज → केवल कंटेंट
contextzip err node server.js              # एरर-केंद्रित आउटपुट

# एनालिटिक्स:
contextzip gain                  # डैशबोर्ड
contextzip gain --by-feature     # फ़िल्टर-वार आँकड़े
contextzip gain --graph          # दैनिक चार्ट
contextzip gain --history        # हाल के कमांड

# प्रबंधन:
contextzip init --show           # सेटअप जाँचें
contextzip update                # सेल्फ़-अपडेट
contextzip uninstall             # साफ़ अनइंस्टॉल
```

---

## 🤝 योगदान करें

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 टेलीमेट्री

ContextZip टूल को बेहतर बनाने के लिए अनाम उपयोग आँकड़े (कमांड गणना, बचत प्रतिशत) एकत्र करता है। कोई व्यक्तिगत डेटा या कमांड सामग्री प्रेषित नहीं की जाती।

**अक्षम करें:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# या ~/.config/contextzip/config.toml में:
# [telemetry]
# enabled = false
```

## 📜 लाइसेंस

MIT — rtk-ai द्वारा [RTK](https://github.com/rtk-ai/rtk) का Fork।

---

<p align="center">
  <b>⚡ कम शोर। ज़्यादा कोड। तेज़ डिलीवरी।</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
