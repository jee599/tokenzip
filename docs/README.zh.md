# TokenZip

> Claude Code 上下文优化工具。减少 60-90% 的 LLM token 消耗。

[English](../README.md) | [한국어](README.ko.md) | [日本語](README.ja.md) | [中文](#)

## 功能

TokenZip 包装你的 CLI 命令，在输出到达 Claude Code 上下文窗口之前进行压缩。噪音减少 = 更多空间留给实际代码。

基于 [RTK](https://github.com/rtk-ai/rtk) 构建，增加了 6 个 RTK 未能捕获的噪音过滤器。

## 安装

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | bash
```

就这样。重启 Claude Code。

## 压缩对象

| 噪音来源 | Before | After | 节省率 |
|---|---|---|---|
| 错误堆栈跟踪 | 30 行 node_modules 帧 | 3 行：错误 + 你的代码 | ~93% |
| 网页抓取 | 3,000 token 的导航/页脚/广告 | 800 token 的内容 | ~73% |
| ANSI/加载动画 | 转义码、进度条 | 干净文本 | ~85% |
| 构建错误 | 40 个相同的 TS2322 错误 | 按错误码分组，保留所有位置 | ~81% |
| 包安装 | 150 行弃用/赞助信息 | 3 行：摘要 + 安全警告 | ~95% |
| Docker 构建 | 50 行层哈希 | 1 行：✓ built app:latest | ~96% |
| CLI 输出 | git/test/ls 噪音 | 压缩（经由 RTK） | ~78% |

## Before / After

### 错误堆栈跟踪
**Before**（30 行，约 1,500 token）：
```
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    ... 25 more node_modules frames
```

**After**（3 行，约 100 token）：
```
TypeError: Cannot read properties of undefined (reading 'id')
  → /app/src/api/users.ts:47         getUserProfile()
  → /app/src/middleware/auth.ts:12    processAuth()
  (+ 27 framework frames hidden)
```

### 包安装
**Before**（150 行，约 2,000 token）：
```
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf v3 is no longer supported
... 47 more deprecated warnings
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
8 vulnerabilities (2 moderate, 6 high)
```

**After**（3 行，约 50 token）：
```
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 moderate)
⚠ deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
```

### Docker 构建（成功）
**Before**（50 行）：包含哈希、缓存行、中间容器的逐步输出
**After**（1 行）：`✓ built my-app:latest (12 steps, 8 cached)`

### Docker 构建（失败）
保留上下文：失败步骤 + 前 2 个步骤 + 完整错误信息 + 退出码。

## CLI

```bash
# 包装命令（通过钩子自动应用）
tokenzip git status
tokenzip cargo test
tokenzip npm install

# 新命令
tokenzip web https://docs.example.com    # 提取页面内容
tokenzip err node server.js              # 错误专注输出

# 分析
tokenzip gain                  # 总节省量
tokenzip gain --by-feature     # 按过滤器类型的节省量
tokenzip gain --graph          # 每日节省量图表
tokenzip gain --history        # 最近命令历史

# 设置
tokenzip init -g               # 全局安装钩子
tokenzip init --show           # 检查安装状态
tokenzip uninstall             # 干净卸载
tokenzip update                # 自更新
```

## 工作原理

1. Claude Code 钩子拦截 bash 命令
2. 命令路由到 TokenZip
3. ANSI 预处理器从所有输出中去除转义码
4. 命令特定过滤器压缩结果
5. 错误后处理器捕获所有输出中的堆栈跟踪
6. 压缩后的输出进入 Claude Code 的上下文

## 配置

```bash
# 配置文件
~/.config/tokenzip/config.toml

# 项目级过滤器
.tokenzip/filters.toml
```

## 系统要求

- Claude Code（或任何使用 PreToolUse 钩子的工具）
- macOS（arm64/x86_64）或 Linux（x86_64）

## 归属

基于 [RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk) 构建，由 rtk-ai 开发。MIT 许可证。
