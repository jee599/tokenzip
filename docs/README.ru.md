<h1 align="center">
  <br>
  ⚡ ContextZip
  <br>
</h1>

<h3 align="center">
  Вывод Claude полон ненужного шума.<br>
  ContextZip сжимает его на 60-90%. <code>npx contextzip</code> → 5 секунд (первый запуск скачивает бинарник).
</h3>

<p align="center">
  <a href="https://github.com/jee599/contextzip/releases"><img src="https://img.shields.io/github/v/release/jee599/contextzip?style=flat-square&color=blue" alt="Release" /></a>
  <a href="https://github.com/jee599/contextzip/actions"><img src="https://img.shields.io/github/actions/workflow/status/jee599/contextzip/ci.yml?style=flat-square" alt="CI" /></a>
  <img src="https://img.shields.io/badge/tests-1%2C056_passing-brightgreen?style=flat-square" alt="Tests" />
  <img src="https://img.shields.io/badge/benchmarks-102_cases-orange?style=flat-square" alt="Benchmarks" />
  <a href="LICENSE"><img src="https://img.shields.io/github/license/jee599/contextzip?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="#-установить-сейчас">Установка</a> •
  <a href="#-посмотрите-разницу">Before/After</a> •
  <a href="#-цифры-не-врут">Бенчмарк</a> •
  <a href="../README.md">English</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.pt.md">Português</a> •
  <a href="README.hi.md">हिन्दी</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.de.md">Deutsch</a> •
  Русский •
  <a href="README.tr.md">Türkçe</a> •
  <a href="README.vi.md">Tiếng Việt</a>
</p>

---

```
  30 строк стектрейса node_modules          →    3 строки
  150 строк предупреждений npm deprecated   →    3 строки
  50 строк хешей Docker build               →    1 строка
  ANSI-цвета, спиннеры, прогресс-бары       →    удалены
```

<h3 align="center">⬇️ Одна строка. Всё.</h3>

```bash
npx contextzip
```

<p align="center">Перезапустите Claude Code. Каждая команда сжимается автоматически. Без настройки.<br>
<b>macOS · Linux · Windows</b></p>

<details>
<summary>Другие способы установки</summary>

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
brew install jee599/tap/contextzip

# Windows (PowerShell)
npx contextzip

# Разработчики на Rust
cargo install --git https://github.com/jee599/contextzip
```

</details>

---

## 👀 Посмотрите Разницу

### 💥 Ошибка Node.js — 30 строк → 3 строки (92% экономии)

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

### 📦 npm install — 150 строк → 3 строки (58-95% экономии, зависит от количества предупреждений)

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




Безопасность сохранена. Шум удалён.

💾 saved 95%
```

</td>
</tr>
</table>

### 🐳 Docker Build — 50 строк → 1 строка (96% экономии)

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






Безопасность сохранена. Шум удалён.

💾 saved 96%
```

</td>
</tr>
</table>

### 🐍 Python Traceback — фреймворк-фреймы скрыты (72% экономии)

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

### 🦀 Rust Panic — std/tokio удалены (2-7% экономии через err-фильтр)

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


💾 saved 2-7% (err-фильтр; формат Rust panic ещё не оптимизирован)
```

</td>
</tr>
</table>

### 🔨 TypeScript Build — 40 ошибок сгруппировано (81% экономии)

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

### 🌐 Веб-страница — nav/footer/реклама удалены (73% экономии)

<table>
<tr>
<td width="50%">

**❌ Before (вывод curl)**
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
соцсети — всё удалено.
💾 saved 73%
```

</td>
</tr>
</table>

### 🎨 ANSI / Спиннеры — невидимый шум удалён (83% экономии)

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




Только финальные состояния сохранены.

💾 saved 83%
```

</td>
</tr>
</table>

<details>
<summary><b>🐳 Сбой Docker — контекст сохранён</b></summary>

```
✗ Docker build failed at step 7/12

Step 5/12 : COPY package*.json ./    (cached ✓)
Step 6/12 : RUN npm install          (cached ✓)
Step 7/12 : RUN npm run build        ← FAILED
  error: Module not found: 'react-dom/client'
  Exit code: 1
```

> Упавший шаг + 2 предыдущих шага + сообщение об ошибке + код выхода. Всегда.

</details>

<details>
<summary><b>☕ Java / 🐹 Go стектрейсы</b></summary>

**Java** — удаляет `java.lang.reflect`, `sun.reflect`, `org.springframework`, `org.apache`, `jdk.internal`:
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

**Go** — удаляет `runtime/`, `runtime.gopanic`, `runtime.main`:
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

## 📊 Цифры Не Врут

> **102 реальных теста. Без подтасовки.**

| Категория | Тесты | Средняя экономия | 🏆 Лучший | 💀 Худший |
|:---------|------:|:----------:|:-------:|:-------:|
| 🐳 Docker build | 10 | **88%** | 97% | 77% |
| 🎨 ANSI/спиннеры | 15 | **83%** | 98% | 0% |
| 💥 Трассировки ошибок | 20 | **59%** | 97% | -12% |
| 🔨 Ошибки сборки | 15 | **56%** | 90% | -10% |
| 🌐 Веб-страницы | 15 | **43%** | 64% | 5% |
| 💻 CLI-команды | 12 | **42%** | 99% | -56% |
| 📦 Установка пакетов | 15 | **39%** | 99% | 2% |

**Взвешенный итог: 61% экономии** → 326K chars in, 127K chars out

> [!NOTE]
> Отрицательные = вывод увеличился. Бывает на крошечных входных данных. Худшие цифры в таблице, потому что скрывать их было бы нечестно. [Полный бенчмарк →](benchmark-results.md)

---

## 🏎️ Как Это Работает

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

## 🆚 Почему Не Просто RTK?

Построен на [RTK](https://github.com/rtk-ai/rtk) (28k⭐). Все 34 команды RTK включены. **Плюс:**

| | RTK | ContextZip |
|:---|:---:|:---:|
| Сжатие CLI (git, test, ls) | ✅ | ✅ |
| Стектрейсы ошибок (Node/Python/Rust/Go/Java) | ❌ | ✅ |
| Извлечение контента веб-страниц | ❌ | ✅ |
| Удаление ANSI / спиннеров / декораций | 🟡 | ✅ |
| Группировка ошибок сборки (tsc/eslint/cargo) | 🟡 | ✅ |
| Шум установки пакетов (npm/pip/cargo) | ❌ | ✅ |
| Сжатие Docker build | 🟡 | ✅ |
| Экономия по каждой команде | ❌ | ✅ |

---

## 📈 Отслеживайте Всё

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
  <code>--graph</code> дневной график &nbsp;•&nbsp; <code>--history</code> последние команды
</p>

---

## 🛡️ Ничего Важного Не Теряется

| | |
|:---|:---|
| 🔴 Сообщения об ошибках | **ВСЕГДА** сохраняются |
| 📍 Файл:строка в ошибках сборки | **НИКОГДА** не удаляются |
| 🔒 Предупреждения безопасности (CVE, GHSA) | **ВСЕГДА** сохраняются |
| 🐳 Контекст сбоя Docker | **ВСЕГДА** сохраняется |
| ⏎ Коды выхода | **ВСЕГДА** передаются |

> [!IMPORTANT]
> ContextZip удаляет только **подтверждённый шум**. При сомнении → passthrough.

---

## 🔧 Команды

```bash
# Автоматически (хук преобразует — без префикса):
git status              npm install             cargo test
docker build .          pip install flask        go test ./...

# Вручную:
contextzip web https://docs.example.com    # страница → только контент
contextzip err node server.js              # вывод с фокусом на ошибки

# Аналитика:
contextzip gain                  # дашборд
contextzip gain --by-feature     # статистика по фильтрам
contextzip gain --graph          # дневной график
contextzip gain --history        # последние команды

# Управление:
contextzip init --show           # проверить настройку
contextzip update                # обновить
contextzip uninstall             # чистое удаление
```

---

## 🤝 Внести Вклад

```bash
git clone https://github.com/jee599/contextzip.git && cd contextzip
cargo test         # 1,056 tests
cargo clippy       # lint
```

## 📡 Телеметрия

ContextZip собирает анонимную статистику использования (количество команд, процент экономии) для улучшения инструмента. Личные данные и содержимое команд не передаются.

**Отключить:**
```bash
export CONTEXTZIP_TELEMETRY_DISABLED=1
# или в ~/.config/contextzip/config.toml:
# [telemetry]
# enabled = false
```

## 📜 Лицензия

MIT — Fork [RTK](https://github.com/rtk-ai/rtk) от rtk-ai.

---

<p align="center">
  <b>⚡ Меньше шума. Больше кода. Деплой быстрее.</b>
</p>

<p align="center">
  <a href="https://github.com/jee599/contextzip">
    <img src="https://img.shields.io/badge/GitHub-⭐_Star_this_repo-yellow?style=for-the-badge&logo=github" alt="Star" />
  </a>
</p>

[![Star History Chart](https://api.star-history.com/svg?repos=jee599/contextzip&type=Date)](https://star-history.com/#jee599/contextzip&Date)
