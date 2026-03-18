# TokenZip — 구현 디자인

## 개요

RTK(rtk-ai/rtk) 풀 포크 기반 Claude Code 컨텍스트 최적화 도구.
RTK의 34개 CLI 압축 모듈 전부 포함 + 6개 신규/강화 노이즈 필터 추가.

## 결정 사항

- **이름**: tokenzip (레포/crate/바이너리 통일)
- **전략**: RTK 소스 풀 포크 → rtk→tokenzip 리네이밍
- **스코프**: 3주, 주 단위 검증 게이트
- **라이선스**: MIT (RTK 원본 저작권 유지 + "Based on rtk by rtk-ai")

## 아키텍처

```
사용자/Claude Code
  ↓ 명령어
PreToolUse 훅 → tokenzip-rewrite.sh → tokenzip 바이너리
  ├── [1] ANSI 전처리 (ansi_filter.rs) — 모든 출력에 먼저 적용
  ├── [2] 명령어 라우터 (main.rs, 기존 RTK 구조)
  │     ├── 기존 34개 모듈 (git, test, ls, grep, cargo 등)
  │     ├── error_cmd.rs (신규) — 에러 스택트레이스 압축
  │     ├── web_cmd.rs (신규) — 웹 페이지 본문 추출
  │     ├── build_cmd.rs (신규) — 빌드 에러 그룹화
  │     ├── pkg_cmd.rs (신규) — 패키지 설치 로그 압축
  │     └── docker_cmd.rs (강화) — Docker 빌드 로그 압축
  ├── [3] 출력 후처리 — 에러 스택트레이스 자동 감지
  └── [4] SQLite 트래킹 (feature 컬럼 추가)
```

## 리네이밍 범위

- Cargo.toml: name/bin = "tokenzip"
- 데이터: ~/.local/share/tokenzip/ (Linux), ~/Library/Application Support/tokenzip/ (macOS)
- 설정: ~/.config/tokenzip/config.toml
- 훅: ~/.claude/hooks/tokenzip-rewrite.sh
- 소스 내 모든 "rtk" 참조 → "tokenzip"

## 신규 모듈 설계

### ansi_filter.rs — ANSI/스피너/장식 전처리
- ANSI 이스케이프: `\x1b\[[0-9;]*[a-zA-Z]`
- 스피너 유니코드: `⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏`
- 프로그레스 바: 블록 문자 반복 + 퍼센트 (최종 상태 보존)
- 장식 구분선: 동일 문자 5회+ 연속
- 캐리지 리턴: 마지막 상태만 보존
- 보존: 타임스탬프 줄, error/warn/fail 포함 줄

### error_cmd.rs — 에러 스택트레이스 압축
- 감지: Node.js(`at `+파일), Python(`Traceback`/`File "`), Rust(`thread '`+`panicked`), Go(`goroutine`), Java(`at `+패키지.클래스)
- 제거: node_modules/, site-packages/, java.lang.reflect., runtime/, std::rt::
- 보존: 에러 메시지 첫 줄, 사용자 코드 프레임, `(+ N framework frames hidden)`
- 반복 에러: 같은 메시지 N회 → 첫 번째 + `(repeated N times)`

### web_cmd.rs — 웹 페이지 압축
- 제거: nav, header, footer, aside, script, style, noscript, cookie/ad/social 관련
- 보존: main, article, pre, code, table, img alt
- 구현: scraper crate로 HTML 태그/클래스 기반 규칙 매칭
- 적용: curl/wget HTML 감지 시 자동, `tokenzip web <url>` 명시적

### build_cmd.rs — 빌드 에러 그룹화
- 대상: tsc(TS+숫자), ESLint(규칙명), cargo(E+4자리), mypy, pylint
- 같은 에러 코드 그룹화, 메시지 1회 표시
- 안전: 모든 파일명+줄 번호 유지

### pkg_cmd.rs — 패키지 설치 로그 압축
- 대상: npm, yarn, pnpm, pip, cargo
- 제거: deprecated(보안 제외), funding, 의존성 해결, progress, cached
- 보존: vulnerability, security, critical, CVE-, GHSA- 포함 경고

### docker_cmd.rs — Docker 빌드 로그 압축
- 성공: 1줄 요약 `✓ built image:tag (N steps, M cached)`
- 실패: 실패 단계 + 직전 2단계 + 에러 메시지 + exit code 보존

## 트래킹 확장

```sql
ALTER TABLE tracking ADD COLUMN feature TEXT DEFAULT 'cli';
-- 값: 'cli', 'error', 'web', 'ansi', 'build', 'pkg', 'docker'
```

`tokenzip gain --by-feature` 서브커맨드 추가.

## 설치

- `curl -fsSL https://raw.githubusercontent.com/jee599/tokenzip/main/install.sh | bash`
- install.sh: OS/아키텍처 감지 → 바이너리 다운로드 → 훅 자동 등록
- RTK 감지 시 교체/공존/취소 선택
- `tokenzip uninstall` / `tokenzip update`

## 구현 순서

- Week 1: RTK 포크 + 리네이밍 + 설치 스크립트 + CI/CD + 트래킹 확장 → 검증
- Week 2: ansi_filter + error_cmd + web_cmd → 검증
- Week 3: build_cmd + pkg_cmd + docker_cmd + 통합 테스트 → 최종 검증

## 완료 기준

스펙 문서의 16개 완료 기준 전부 충족.
