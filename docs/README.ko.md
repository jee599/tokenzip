# TokenZip

> Claude Code 컨텍스트 최적화 도구. LLM 토큰 소비를 60-90% 줄인다.

[English](../README.md) | [한국어](#) | [日本語](README.ja.md) | [中文](README.zh.md)

## 하는 일

TokenZip은 CLI 명령어 출력을 압축해서 Claude Code 컨텍스트 윈도우에 전달한다. 노이즈가 줄면 실제 코드에 쓸 공간이 늘어난다.

[RTK](https://github.com/rtk-ai/rtk) 기반이며, RTK가 잡지 못하는 노이즈를 위한 필터 6개를 추가했다.

## 설치

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | bash
```

끝. Claude Code를 재시작한다.

## 압축 대상

| 노이즈 소스 | Before | After | 절감률 |
|---|---|---|---|
| 에러 스택트레이스 | node_modules 프레임 30줄 | 에러 + 내 코드 3줄 | ~93% |
| 웹 페이지 fetch | nav/footer/광고 포함 3,000 토큰 | 본문만 800 토큰 | ~73% |
| ANSI/스피너 | 이스케이프 코드, 프로그레스 바 | 깨끗한 텍스트 | ~85% |
| 빌드 에러 | 동일한 TS2322 에러 40개 | 에러 코드별 그룹화, 위치 전부 보존 | ~81% |
| 패키지 설치 | deprecated/funding 150줄 | 요약 + 보안 경고 3줄 | ~95% |
| Docker 빌드 | 레이어 해시 50줄 | ✓ built app:latest 1줄 | ~96% |
| CLI 출력 | git/test/ls 노이즈 | 압축 (RTK 경유) | ~78% |

## Before / After

### 에러 스택트레이스
**Before** (30줄, ~1,500 토큰):
```
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    at next (/app/node_modules/express/lib/router/route.js:144:13)
    ... 25 more node_modules frames
```

**After** (3줄, ~100 토큰):
```
TypeError: Cannot read properties of undefined (reading 'id')
  → /app/src/api/users.ts:47         getUserProfile()
  → /app/src/middleware/auth.ts:12    processAuth()
  (+ 27 framework frames hidden)
```

### 패키지 설치
**Before** (150줄, ~2,000 토큰):
```
npm warn deprecated inflight@1.0.6: This module is not supported
npm warn deprecated rimraf@3.0.2: Rimraf v3 is no longer supported
... 47 more deprecated warnings
added 847 packages, and audited 848 packages in 32s
143 packages are looking for funding
8 vulnerabilities (2 moderate, 6 high)
```

**After** (3줄, ~50 토큰):
```
✓ 847 packages (32s)
⚠ 8 vulnerabilities (6 high, 2 moderate)
⚠ deprecated bcrypt@3.0.0: security vulnerability (CVE-2023-31484)
```

### Docker 빌드 (성공)
**Before** (50줄): 해시, 캐시 라인, 중간 컨테이너가 포함된 단계별 출력
**After** (1줄): `✓ built my-app:latest (12 steps, 8 cached)`

### Docker 빌드 (실패)
컨텍스트 보존: 실패 단계 + 이전 2단계 + 전체 에러 메시지 + 종료 코드.

## CLI

```bash
# 래핑된 명령어 (훅으로 자동 적용)
tokenzip git status
tokenzip cargo test
tokenzip npm install

# 새 명령어
tokenzip web https://docs.example.com    # 페이지 콘텐츠 추출
tokenzip err node server.js              # 에러 중심 출력

# 분석
tokenzip gain                  # 전체 절감량
tokenzip gain --by-feature     # 필터 유형별 절감량
tokenzip gain --graph          # 일별 절감량 차트
tokenzip gain --history        # 최근 명령어 히스토리

# 설정
tokenzip init -g               # 훅 글로벌 설치
tokenzip init --show           # 설치 상태 확인
tokenzip uninstall             # 깔끔한 제거
tokenzip update                # 셀프 업데이트
```

## 동작 방식

1. Claude Code 훅이 bash 명령어를 가로챈다
2. 명령어가 TokenZip을 거친다
3. ANSI 전처리기가 모든 출력에서 이스케이프 코드를 제거한다
4. 명령어별 필터가 결과를 압축한다
5. 에러 후처리기가 모든 출력에서 스택트레이스를 잡는다
6. 압축된 출력이 Claude Code 컨텍스트로 전달된다

## 설정

```bash
# 설정 파일
~/.config/tokenzip/config.toml

# 프로젝트 수준 필터
.tokenzip/filters.toml
```

## 요구 사항

- Claude Code (또는 PreToolUse 훅을 사용하는 도구)
- macOS (arm64/x86_64) 또는 Linux (x86_64)

## 출처

[RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk) 기반. rtk-ai 제작. MIT 라이선스.
