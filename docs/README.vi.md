<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Output của Claude đầy rác không cần thiết.<br>
  ContextZip nén 60-90%. <code>npx contextzip</code> → 5 giây (lần chạy đầu tải binary).
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-cài-đặt-ngay">Cài đặt</a> •
  <a href="#-xem-sự-khác-biệt">Before/After</a> •
  <a href="#-con-số-không-nói-dối">Benchmark</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.hi.md">हिन्दी</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ru.md">Русский</a> •
  <a href="README.tr.md">Türkçe</a> •
  Tiếng Việt
</p>

---

```
  30 dòng stacktrace node_modules           →    3 dòng
  150 dòng cảnh báo npm deprecated          →    3 dòng
  50 dòng hash Docker build                 →    1 dòng
  Màu ANSI, spinner, thanh tiến trình       →    xóa sạch
```

<h3 align="center">⬇️ Một dòng. Chỉ vậy thôi.</h3>

```bash
npx contextzip
```

<p align="center">Khởi động lại Claude Code. Mọi lệnh tự động được nén. Không cần cấu hình.<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>Các cách cài đặt khác</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Lập trình viên Rust
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 Xem Sự Khác Biệt

### 💥 Lỗi Node.js — 30 dòng → 3 dòng (tiết kiệm 92%)

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

### 📦 npm install — 150 dòng → 3 dòng (tiết kiệm 58-95%, tùy số lượng cảnh báo)

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




Bảo mật giữ nguyên. Rác đã dọn.

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker Build — 50 dòng → 1 dòng (tiết kiệm 96%)

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






Bảo mật giữ nguyên. Rác đã dọn.

💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — ẩn frame framework (tiết kiệm 72%)

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

### 🦀 Rust Panic — xóa std/tokio (tiết kiệm 2-7% qua bộ lọc err)

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


💾 saved 2-7% (bộ lọc err; định dạng Rust panic chưa được tối ưu)
```

</td>
</tr>
</table>

### 🔨 TypeScript Build — gom nhóm 40 lỗi (tiết kiệm 81%)

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

### 🌐 Trang web — xóa nav/footer/quảng cáo (tiết kiệm 73%)

<table>
<tr>
<td width="50%">

**❌ Before (output curl)**
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
liên kết mạng xã hội — tất cả đã xóa.
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / Spinner — xóa rác vô hình (tiết kiệm 83%)

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




Chỉ giữ lại trạng thái cuối cùng.

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Docker thất bại — bối cảnh được giữ lại</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> Bước thất bại + 2 bước trước + thông báo lỗi + mã thoát. Luôn luôn.

</details>

<details>
<summary><b>☕ Java / 🐹 Go stacktrace</b></summary>

**Java** — xóa `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal`:
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

**Go** — xóa `runtime/`, `runtime.gopanic`, `runtime.main`:
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

## 📊 Con Số Không Nói Dối

> **102 bài test thực tế. Không cherry-picking.**

| Danh mục | Bài test | Tiết kiệm TB | 🏆 Tốt nhất | 💀 Tệ nhất |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker build | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/spinner | 15 | **83%** | 98% | 0% |
| 💥 Error trace | 20 | **59%** | 97% | -12% |
| 🔨 Build error | 15 | **56%** | 90% | -10% |
| 🌐 Trang web | 15 | **43%** | 64% | 5% |
| 💻 Lệnh CLI | 12 | **42%** | 99% | -56% |
| 📦 Cài đặt package | 15 | **39%** | 99% | 2% |

**Tổng có trọng số: tiết kiệm 61%** → 326K chars in, 127K chars out

> [!NOTE]
> Âm = output tăng lên. Xảy ra với input rất nhỏ. Chúng tôi đưa con số tệ nhất vào bảng vì giấu đi là không trung thực. [Benchmark đầy đủ →](benchmark-results.md)

---

## 🏎️ Cách Hoạt Động

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

## 🆚 Tại Sao Không Chỉ Dùng RTK?

Xây dựng trên [RTK](https://github.com/rtk-ai/rtk) (28k⭐). Bao gồm toàn bộ 34 lệnh RTK. **Thêm vào đó:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| Nén CLI (git, test, ls) | ✅ | ✅ |
| Stacktrace lỗi (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| Trích xuất nội dung web | ❌ | ✅ |
| Xóa ANSI / spinner / trang trí | 🟡 | ✅ |
| Gom nhóm lỗi build (tsc/eslint/cargo) | 🟡 | ✅ |
| Rác cài đặt package (npm/pip/cargo) | ❌ | ✅ |
| Nén Docker build | 🟡 | ✅ |
| Hiển thị tiết kiệm theo lệnh | ❌ | ✅ |

---

## 📈 Theo Dõi Mọi Thứ

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
  <code>--graph</code> biểu đồ hàng ngày &nbsp;•&nbsp; <code>--history</code> lệnh gần đây
</p>

---

## 🛡️ Không Mất Gì Quan Trọng

| | |
|:---|:---|
| 🔴 Thông báo lỗi | **LUÔN** được giữ |
| 📍 File:dòng trong lỗi build | **KHÔNG BAO GIỜ** bị xóa |
| 🔒 Cảnh báo bảo mật (CVE, GHSA) | **LUÔN** được giữ |
| 🐳 Bối cảnh Docker thất bại | **LUÔN** được giữ |
| ⏎ Mã thoát | **LUÔN** được truyền |

> [!IMPORTANT]
> ContextZip chỉ xóa **rác đã xác nhận**. Khi nghi ngờ → passthrough.

---

## 🔧 Lệnh

```bash
# Tự động (hook chuyển đổi — không cần tiền tố):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# Thủ công:
contextzip web https://docs.example.com    # trang → chỉ nội dung
contextzip err node server.js              # output tập trung vào lỗi

# Phân tích:
contextzip gain                  # bảng điều khiển
contextzip gain --by-feature     # thống kê theo bộ lọc
contextzip gain --graph          # biểu đồ hàng ngày
contextzip gain --history        # lệnh gần đây

# Quản lý:
contextzip init --show           # kiểm tra cài đặt
contextzip update                # tự cập nhật
contextzip uninstall             # gỡ cài đặt sạch
```

---

## 🤝 Đóng Góp

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 Telemetry

ContextZip thu thập thống kê sử dụng ẩn danh (số lệnh, phần trăm tiết kiệm) để cải thiện công cụ. Không truyền dữ liệu cá nhân hay nội dung lệnh.

**Tắt:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# hoặc trong ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 Giấy phép

MIT — Fork của [RTK](https://github.com/rtk-ai/rtk) bởi rtk-ai.

---

<p align="center">
  <b>⚡ Ít rác hơn. Nhiều code hơn. Deploy nhanh hơn.</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
