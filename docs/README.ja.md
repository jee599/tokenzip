<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Claudeの出力は不要なノイズだらけ。<br>
  ContextZipが60-90%圧縮する。<code>npx contextzip</code> → 5秒（初回はバイナリをダウンロード）。
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-今すぐインストール">インストール</a> •
  <a href="#-違いを見よ">Before/After</a> •
  <a href="#-数字が証明する">ベンチマーク</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  日本語 •
  <a href="README.zh.md">中文</a>
</p>

---

```
  30行のnode_modulesスタックトレース    →    3行
  150行のnpm deprecatedの警告          →    3行
  50行のDockerビルドハッシュ            →    1行
  ANSIカラー、スピナー、プログレスバー  →    削除
```

<h3 align="center">⬇️ 1行で完了。</h3>

```bash
npx contextzip
```

<p align="center">Claude Codeを再起動。すべてのコマンドが自動圧縮される。設定不要。<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>他のインストール方法</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Rust開発者
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 違いを見よ

### 💥 Node.jsエラー — 30行 → 3行（92%削減）

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

### 📦 npm install — 150行 → 3行（58-95%削減、警告数により変動）

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

### 🐳 Dockerビルド — 50行 → 1行（96%削減）

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





セキュリティ情報は保持。残りは削除。


💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — フレームワークフレームを非表示（72%削減）

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

### 🦀 Rust Panic — std/tokioを除去（errフィルターで2-7%削減）

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


💾 saved 2-7% (errフィルター; Rust panicフォーマット未最適化)
```

</td>
</tr>
</table>

### 🔨 TypeScriptビルド — 40エラーをグループ化（81%削減）

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

### 🌐 Webページ — nav/footer/広告を除去（73%削減）

<table>
<tr>
<td width="50%">

**❌ Before (curl出力)**
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
ソーシャルリンク — すべて除去。
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / スピナー — 見えないノイズを除去（83%削減）

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




最終状態のみ保持。

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Docker失敗 — コンテキスト保持</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> 失敗ステップ + 前2ステップ + エラーメッセージ + 終了コード。常に。

</details>

<details>
<summary><b>☕ Java / 🐹 Goスタックトレース</b></summary>

**Java** — `java.lang.reflect`、`sun.reflect`、`org.springframework`、`org.apache`、`jdk.internal`を除去：
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

**Go** — `runtime/`、`runtime.gopanic`、`runtime.main`を除去：
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

## 📊 数字が証明する

> **102件の実戦テスト。チェリーピッキングなし。**

| カテゴリ | テスト | 平均削減 | 🏆 最高 | 💀 最低 |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Dockerビルド | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/スピナー | 15 | **83%** | 98% | 0% |
| 💥 エラートレース | 20 | **59%** | 97% | -12% |
| 🔨 ビルドエラー | 15 | **56%** | 90% | -10% |
| 🌐 Webページ | 15 | **43%** | 64% | 5% |
| 💻 CLIコマンド | 12 | **42%** | 99% | -56% |
| 📦 パッケージインストール | 15 | **39%** | 99% | 2% |

**加重平均: 61%削減** → 326K chars in, 127K chars out

> [!NOTE]
> マイナス = 出力が増加。極小入力で発生する。最低値も隠さず公開している。[全ベンチマーク →](benchmark-results.md)

---

## 🏎️ 仕組み

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

## 🆚 なぜRTKだけでは不十分か？

[RTK](https://github.com/rtk-ai/rtk)（28k⭐）ベース。RTKの全34コマンドを含む。**さらに：**

| | RTK | ContextZip |
|:---|:---:|:---:|
| CLI圧縮 (git, test, ls) | ✅ | ✅ |
| エラースタックトレース (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| Webページコンテンツ抽出 | ❌ | ✅ |
| ANSI / スピナー / デコレーション除去 | 🟡 | ✅ |
| ビルドエラーグルーピング (tsc/eslint/cargo) | 🟡 | ✅ |
| パッケージインストールノイズ (npm/pip/cargo) | ❌ | ✅ |
| Dockerビルド圧縮 | 🟡 | ✅ |
| コマンド別削減量表示 | ❌ | ✅ |

---

## 📈 すべてを追跡する

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
  <code>--graph</code> 日別チャート &nbsp;•&nbsp; <code>--history</code> 最近のコマンド
</p>

---

## 🛡️ 重要な情報は決して失わない

| | |
|:---|:---|
| 🔴 エラーメッセージ | **常に**保持 |
| 📍 ビルドエラーのファイル:行番号 | **絶対に**除去しない |
| 🔒 セキュリティ警告 (CVE, GHSA) | **常に**保持 |
| 🐳 Docker失敗コンテキスト | **常に**保持 |
| ⏎ 終了コード | **常に**伝播 |

> [!IMPORTANT]
> ContextZipは**確認済みのノイズのみ**除去する。判断に迷う場合、元の出力をそのまま通す。

---

## 🔧 コマンド

```bash
# 自動（フックが変換 — プレフィックス不要）：
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# 手動：
contextzip web https://docs.example.com    # ページ → コンテンツのみ
contextzip err node server.js              # エラー特化出力

# 分析：
contextzip gain                  # ダッシュボード
contextzip gain --by-feature     # フィルター別統計
contextzip gain --graph          # 日別チャート
contextzip gain --history        # 最近のコマンド

# 管理：
contextzip init --show           # セットアップ確認
contextzip update                # セルフアップデート
contextzip uninstall             # クリーンアンインストール
```

---

## 🤝 コントリビュート

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 テレメトリ

ContextZipは匿名の使用統計（コマンド数、削減率）を収集し、ツールの改善に活用する。個人情報やコマンド内容は送信しない。

**無効化：**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# または ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 ライセンス

MIT — [RTK](https://github.com/rtk-ai/rtk) by rtk-aiのフォーク。

---

<p align="center">
  <b>⚡ ノイズを減らし、コードに集中し、もっと速くシップ。</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
