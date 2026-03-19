<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Claude 的输出充满了无用噪音。<br>
  ContextZip 压缩 60-90%。<code>npx contextzip</code> → 5 秒（首次运行下载二进制文件）。
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-立即安装">安装</a> •
  <a href="#-看看区别">Before/After</a> •
  <a href="#-数字说明一切">基准测试</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  中文
</p>

---

```
  30 行 node_modules 堆栈帧           →    3 行
  150 行 npm deprecated 警告          →    3 行
  50 行 Docker 构建哈希               →    1 行
  ANSI 颜色、加载动画、进度条          →    删除
```

<h3 align="center">⬇️ 一行搞定。</h3>

```bash
npx contextzip
```

<p align="center">重启 Claude Code。所有命令自动压缩。零配置。<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>其他安装方式</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Rust 开发者
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 看看区别

### 💥 Node.js 报错 — 30 行 → 3 行（节省 93%）

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



💾 saved 93%
```

</td>
</tr>
</table>

### 📦 npm install — 150 行 → 3 行（节省 95%）

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




Security kept. Noise gone.

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker 构建 — 50 行 → 1 行（节省 96%）

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





安全信息保留。噪音删除。


💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — 隐藏框架帧（节省 72%）

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

### 🦀 Rust Panic — 移除 std/tokio（节省 80%）

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


💾 saved 80%
```

</td>
</tr>
</table>

### 🔨 TypeScript 构建 — 40 个错误分组（节省 81%）

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

### 🌐 网页 — 去除 nav/footer/广告（节省 73%）

<table>
<tr>
<td width="50%">

**❌ Before (curl 输出)**
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




Nav、footer、sidebar、newsletter、
社交链接 — 全部移除。
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / 加载动画 — 移除不可见噪音（节省 83%）

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




只保留最终状态。

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Docker 失败 — 保留上下文</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> 失败步骤 + 前 2 步 + 错误信息 + 退出码。始终保留。

</details>

<details>
<summary><b>☕ Java / 🐹 Go 堆栈</b></summary>

**Java** — 移除 `java.lang.reflect`、`sun.reflect`、`org.springframework`、`org.apache`、`jdk.internal`：
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

**Go** — 移除 `runtime/`、`runtime.gopanic`、`runtime.main`：
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

## 📊 数字说明一切

> **102 个实战测试。没有挑选。**

| 类别 | 测试 | 平均节省 | 🏆 最高 | 💀 最低 |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker 构建 | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/加载动画 | 15 | **83%** | 98% | 41% |
| 💥 错误堆栈 | 20 | **59%** | 97% | 2% |
| 🔨 构建错误 | 15 | **56%** | 90% | -10% |
| 🌐 网页 | 15 | **43%** | 64% | 5% |
| 💻 CLI 命令 | 12 | **42%** | 78% | -56% |
| 📦 包安装 | 15 | **39%** | 99% | 2% |

**加权平均: 61% 节省** → 326K chars in, 127K chars out

> [!NOTE]
> 负数 = 输出增大。极小输入时发生。最低值也如实公开，因为隐瞒是不诚实的。[完整基准测试 →](benchmark-results.md)

---

## 🏎️ 工作原理

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

## 🆚 为什么不只用 RTK？

基于 [RTK](https://github.com/rtk-ai/rtk)（28k⭐）。包含 RTK 全部 34 个命令。**额外功能：**

| | RTK | ContextZip |
|:---|:---:|:---:|
| CLI 压缩 (git, test, ls) | ✅ | ✅ |
| 错误堆栈 (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| 网页内容提取 | ❌ | ✅ |
| ANSI / 加载动画 / 装饰移除 | 🟡 | ✅ |
| 构建错误分组 (tsc/eslint/cargo) | 🟡 | ✅ |
| 包安装噪音 (npm/pip/cargo) | ❌ | ✅ |
| Docker 构建压缩 | 🟡 | ✅ |
| 逐命令节省量显示 | ❌ | ✅ |

---

## 📈 追踪一切

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
  <code>--graph</code> 每日图表 &nbsp;•&nbsp; <code>--history</code> 最近命令
</p>

---

## 🛡️ 重要信息绝不丢失

| | |
|:---|:---|
| 🔴 错误信息 | **始终**保留 |
| 📍 构建错误的文件:行号 | **绝不**移除 |
| 🔒 安全警告 (CVE, GHSA) | **始终**保留 |
| 🐳 Docker 失败上下文 | **始终**保留 |
| ⏎ 退出码 | **始终**传播 |

> [!IMPORTANT]
> ContextZip 只移除**已确认的噪音**。不确定时，原始输出原样通过。

---

## 🔧 命令

```bash
# 自动（hook 转换 — 无需前缀）：
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# 手动：
contextzip web https://docs.example.com    # 页面 → 仅内容
contextzip err node server.js              # 错误聚焦输出

# 分析：
contextzip gain                  # 仪表盘
contextzip gain --by-feature     # 按过滤器统计
contextzip gain --graph          # 每日图表
contextzip gain --history        # 最近命令

# 管理：
contextzip init --show           # 检查安装状态
contextzip update                # 自更新
contextzip uninstall             # 干净卸载
```

---

## 🤝 贡献

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 遥测

ContextZip 收集匿名使用统计（命令数、节省率）以改进工具。不传输个人信息或命令内容。

**禁用：**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# 或在 ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 许可证

MIT — [RTK](https://github.com/rtk-ai/rtk) by rtk-ai 的 fork。

---

<p align="center">
  <b>⚡ 减少噪音，专注代码，更快交付。</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
