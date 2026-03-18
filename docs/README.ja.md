# TokenZip

> Claude Codeコンテキスト最適化ツール。LLMトークン消費を60-90%削減。

[English](../README.md) | [한국어](README.ko.md) | [日本語](#) | [中文](README.zh.md)

## 概要

TokenZipはCLIコマンドの出力を圧縮し、Claude Codeのコンテキストウィンドウに渡します。ノイズが減れば、実際のコードに使えるスペースが増えます。

[RTK](https://github.com/rtk-ai/rtk)をベースに、RTKがキャッチできないノイズ用のフィルターを6つ追加しました。

## インストール

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | bash
```

以上です。Claude Codeを再起動してください。

## 圧縮対象

| ノイズソース | Before | After | 削減率 |
|---|---|---|---|
| エラースタックトレース | node_modulesフレーム30行 | エラー + ユーザーコード3行 | ~93% |
| Webページfetch | nav/footer/広告含む3,000トークン | コンテンツのみ800トークン | ~73% |
| ANSI/スピナー | エスケープコード、プログレスバー | クリーンテキスト | ~85% |
| ビルドエラー | 同一TS2322エラー40個 | エラーコード別グループ化、全位置保持 | ~81% |
| パッケージインストール | deprecated/funding 150行 | サマリー + セキュリティ3行 | ~95% |
| Dockerビルド | レイヤーハッシュ50行 | ✓ built app:latest 1行 | ~96% |
| CLI出力 | git/test/lsノイズ | 圧縮（RTK経由） | ~78% |

## Before / After

### エラースタックトレース
**Before**（30行、約1,500トークン）:
```
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    ... 25 more node_modules frames
```

**After**（3行、約100トークン）:
```
TypeError: Cannot read properties of undefined (reading 'id')
  → /app/src/api/users.ts:47         getUserProfile()
  → /app/src/middleware/auth.ts:12    processAuth()
  (+ 27 framework frames hidden)
```

### パッケージインストール
**Before**（150行、約2,000トークン）:
```
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf v3 is no longer supported
... 47 more deprecated warnings
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
8 vulnerabilities (2 moderate, 6 high)
```

**After**（3行、約50トークン）:
```
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 moderate)
⚠ deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
```

### Dockerビルド（成功）
**Before**（50行）: ハッシュ、キャッシュ行、中間コンテナを含むステップごとの出力
**After**（1行）: `✓ built my-app:latest (12 steps, 8 cached)`

### Dockerビルド（失敗）
コンテキスト保持: 失敗ステップ + 前2ステップ + 完全なエラーメッセージ + 終了コード。

## CLI

```bash
# ラップされたコマンド（フック経由で自動適用）
tokenzip git status
tokenzip cargo test
tokenzip npm install

# 新コマンド
tokenzip web https://docs.example.com    # ページコンテンツ抽出
tokenzip err node server.js              # エラー特化出力

# 分析
tokenzip gain                  # 総削減量
tokenzip gain --by-feature     # フィルター種別ごとの削減量
tokenzip gain --graph          # 日別削減量チャート
tokenzip gain --history        # 最近のコマンド履歴

# セットアップ
tokenzip init -g               # フックをグローバルインストール
tokenzip init --show           # インストール状態確認
tokenzip uninstall             # クリーンアンインストール
tokenzip update                # セルフアップデート
```

## 仕組み

1. Claude Codeフックがbashコマンドをインターセプト
2. コマンドがTokenZipにルーティングされる
3. ANSIプリプロセッサがすべての出力からエスケープコードを除去
4. コマンド固有のフィルターが結果を圧縮
5. エラーポストプロセッサがすべての出力からスタックトレースをキャッチ
6. 圧縮された出力がClaude Codeのコンテキストに渡される

## 設定

```bash
# 設定ファイル
~/.config/tokenzip/config.toml

# プロジェクトレベルフィルター
.tokenzip/filters.toml
```

## 要件

- Claude Code（またはPreToolUseフックを使用するツール）
- macOS（arm64/x86_64）またはLinux（x86_64）

## 帰属

[RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk)（rtk-ai制作）をベースに構築。MITライセンス。
