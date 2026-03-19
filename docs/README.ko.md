<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Claude의 출력은 불필요한 노이즈로 가득하다.<br>
  ContextZip이 60-90% 압축한다. <code>npx contextzip</code> → 5초 (첫 실행 시 바이너리 다운로드).
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-지금-바로-설치">설치</a> •
  <a href="#-직접-비교하라">Before/After</a> •
  <a href="#-숫자가-증명한다">벤치마크</a> •
  <a href="../README.md">English</a> •
  한국어 •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.es.md">Español</a> •
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
  30줄 node_modules 스택트레이스      →    3줄
  150줄 npm deprecated 경고           →    3줄
  50줄 Docker 빌드 해시               →    1줄
  ANSI 컬러, 스피너, 프로그레스 바    →    제거
```

<h3 align="center">⬇️ 한 줄이면 끝.</h3>

```bash
npx contextzip
```

<p align="center">Claude Code 재시작. 모든 명령어가 자동 압축된다. 설정 없음.<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>다른 설치 방법</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Rust 개발자
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 직접 비교하라

### 💥 Node.js 에러 — 30줄 → 3줄 (92% 절감)

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

### 📦 npm install — 150줄 → 3줄 (58-95% 절감, 경고 수에 따라 변동)

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

### 🐳 Docker 빌드 — 50줄 → 1줄 (96% 절감)

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




보안 정보 유지. 나머지 제거.



💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — 프레임워크 프레임 숨김 (72% 절감)

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

### 🦀 Rust Panic — std/tokio 제거 (2-7% 절감, err 필터 기준)

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


💾 saved 2-7% (err 필터; Rust panic 포맷 미최적화)
```

</td>
</tr>
</table>

### 🔨 TypeScript 빌드 — 40개 에러 그룹화 (81% 절감)

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

### 🌐 웹 페이지 — nav/footer/광고 제거 (73% 절감)

<table>
<tr>
<td width="50%">

**❌ Before (curl 출력)**
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
social links — 전부 제거.
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / 스피너 — 보이지 않는 노이즈 제거 (83% 절감)

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




최종 상태만 유지.

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Docker 실패 — 컨텍스트 보존</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> 실패 단계 + 이전 2단계 + 에러 메시지 + 종료 코드. 항상.

</details>

<details>
<summary><b>☕ Java / 🐹 Go 스택트레이스</b></summary>

**Java** — `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal` 제거:
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

**Go** — `runtime/`, `runtime.gopanic`, `runtime.main` 제거:
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

## 📊 숫자가 증명한다

> **102개 실전 테스트. 체리피킹 없음.**

| 카테고리 | 테스트 | 평균 절감 | 🏆 최고 | 💀 최저 |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker 빌드 | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/스피너 | 15 | **83%** | 98% | 0% |
| 💥 에러 트레이스 | 20 | **59%** | 97% | -12% |
| 🔨 빌드 에러 | 15 | **56%** | 90% | -10% |
| 🌐 웹 페이지 | 15 | **43%** | 64% | 5% |
| 💻 CLI 명령어 | 12 | **42%** | 99% | -56% |
| 📦 패키지 설치 | 15 | **39%** | 99% | 2% |

**가중 평균: 61% 절감** → 326K chars in, 127K chars out

> [!NOTE]
> 음수 = 출력이 늘어남. 입력이 극소량일 때 발생한다. 최저 수치를 숨기지 않고 그대로 공개한다. [전체 벤치마크 →](benchmark-results.md)

---

## 🏎️ 동작 원리

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

## 🆚 RTK와 뭐가 다른가?

[RTK](https://github.com/rtk-ai/rtk) (28k⭐) 기반. RTK 34개 명령어 전부 포함. **추가 기능:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| CLI 압축 (git, test, ls) | ✅ | ✅ |
| 에러 스택트레이스 (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| 웹 페이지 콘텐츠 추출 | ❌ | ✅ |
| ANSI / 스피너 / 장식 제거 | 🟡 | ✅ |
| 빌드 에러 그룹화 (tsc/eslint/cargo) | 🟡 | ✅ |
| 패키지 설치 노이즈 (npm/pip/cargo) | ❌ | ✅ |
| Docker 빌드 압축 | 🟡 | ✅ |
| 명령어별 절감량 표시 | ❌ | ✅ |

---

## 📈 모든 것을 추적한다

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
  <code>--graph</code> 일별 차트 &nbsp;•&nbsp; <code>--history</code> 최근 명령어
</p>

---

## 🛡️ 중요한 건 절대 잃지 않는다

| | |
|:---|:---|
| 🔴 에러 메시지 | **항상** 보존 |
| 📍 빌드 에러의 파일:줄번호 | **절대** 제거 안 함 |
| 🔒 보안 경고 (CVE, GHSA) | **항상** 유지 |
| 🐳 Docker 실패 컨텍스트 | **항상** 보존 |
| ⏎ 종료 코드 | **항상** 전파 |

> [!IMPORTANT]
> ContextZip은 **확인된 노이즈만** 제거한다. 확신이 없으면 원본을 그대로 통과시킨다.

---

## 🔧 명령어

```bash
# 자동 (훅이 변환 — 접두사 불필요):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# 수동:
contextzip web https://docs.example.com    # 페이지 → 콘텐츠만
contextzip err node server.js              # 에러 중심 출력

# 분석:
contextzip gain                  # 대시보드
contextzip gain --by-feature     # 필터별 통계
contextzip gain --graph          # 일별 차트
contextzip gain --history        # 최근 명령어

# 관리:
contextzip init --show           # 설치 상태 확인
contextzip update                # 셀프 업데이트
contextzip uninstall             # 깨끗한 제거
```

---

## 🤝 기여

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 텔레메트리

ContextZip은 익명 사용 통계(명령어 수, 절감률)를 수집해 도구 개선에 활용한다. 개인 정보나 명령어 내용은 전송하지 않는다.

**비활성화:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# 또는 ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 라이선스

MIT — [RTK](https://github.com/rtk-ai/rtk) by rtk-ai 포크.

---

<p align="center">
  <b>⚡ 노이즈는 줄이고. 코드에 집중하고. 더 빠르게 배포하고.</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
