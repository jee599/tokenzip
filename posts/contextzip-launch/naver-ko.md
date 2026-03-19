---
title: "Claude Code가 토큰을 낭비하고 있었다. 코드 한 줄 안 쓰고 3주 만에 고쳤다."
---

한 세션에서 컨텍스트 윈도우를 세 번 날렸다. 세 번째에 참을 수 없었다.

Node.js 앱을 디버깅하고 있었다. Claude가 `npm install`을 돌렸고, deprecated 경고 150줄이 컨텍스트를 채웠다. 이어서 스택트레이스 — `node_modules` 프레임 30줄, 내 코드는 고작 2줄. 그다음 `docker build`가 레이어 해시 50줄을 쏟아냈다.

정작 버그로 돌아갔을 때 Claude는 10분 전에 보여준 코드를 까먹은 상태였다. 컨텍스트가 노이즈로 가득 찼으니까.

그날 밤 RTK를 발견했다. Rust Token Killer. CLI 출력을 압축해서 Claude Code 컨텍스트에 넣기 전에 노이즈를 걸러주는 오픈소스 도구다. 28k 스타, git/test/ls 출력에서 60-90% 절약. 인상적이었다.

근데 에러 스택트레이스는 건드리지 않았다. 웹 페이지도, npm install 노이즈도, Docker 빌드 로그도.

그래서 포크했다.


## 실험: Claude Code가 자기 자신을 위한 도구를 만들 수 있을까?

여기서 이야기가 좀 이상해진다. ContextZip — 40개 모듈, 1,056개 테스트를 가진 Rust CLI — 을 Claude Code로 만들었다. Claude Code의 컨텍스트를 압축하는 도구를, Claude Code가 만든 것이다.

나는 Rust를 쓰지 않았다. 프롬프트를 썼다.

전체 프로젝트는 3주 걸렸다.


## 1주차: RTK 포크, 전체 리네이밍, 인스톨러 완성

첫 작업은 기계적이었다. RTK 소스를 클론하고(34개 커맨드 모듈, 60개 TOML 필터, 950개 테스트), 모든 "rtk" 참조를 "contextzip"으로 바꾸고, 아무것도 안 깨졌는지 확인하는 것.

Claude Code 서브에이전트를 디스패치해서 리네이밍을 시켰다. 70개 파일 변경, 1,544줄 추가, 1,182줄 삭제. 950개 테스트 전부 통과. `--version` 출력은 `contextzip 0.1.0 (based on rtk 0.30.1)`.

그다음 에이전트 3개를 병렬로 돌렸다. 하나는 install.sh 작성, 하나는 5개 플랫폼 빌드용 GitHub Actions CI/CD 세팅, 하나는 SQLite 트래킹 시스템에 `feature` 컬럼 추가.

1주차가 끝났을 때: `curl | bash` 한 줄로 설치 완료, Claude Code 훅 자동 활성화, `contextzip gain --by-feature`로 어떤 필터가 가장 많이 절약했는지 확인 가능.


## 2주차: RTK가 못 잡는 3개 필터

여기서 ContextZip이 RTK와 갈라진다.

에러 스택트레이스부터. Express 미들웨어 프레임 30줄짜리 Node.js 에러가 3줄이 된다. 에러 메시지, 내 코드 프레임, 그리고 "(+ 27 framework frames hidden)". Python은 site-packages를 숨기고, Rust는 std::panicking을, Go는 runtime/을, Java는 java.lang.reflect를 숨긴다.

```
변환 전 (30줄, ~1,500토큰):
TypeError: Cannot read properties of undefined (reading 'id')
    at getUserProfile (/app/src/api/users.ts:47:23)
    at processAuth (/app/src/middleware/auth.ts:12:5)
    at Layer.handle (/app/node_modules/express/lib/router/layer.js:95:5)
    ... node_modules 25줄 더

변환 후 (3줄, ~100토큰):
TypeError: Cannot read properties of undefined (reading 'id')
  → src/api/users.ts:47         getUserProfile()
  → src/middleware/auth.ts:12   processAuth()
  (+ 27 framework frames hidden)
```

93% 절약. Claude는 에러와 내 코드만 본다. Express 내부는 안 본다.

ANSI 전처리기는 모든 명령어 출력에 먼저 적용된다. 이스케이프 코드, 스피너, 프로그레스 바, 장식 구분선을 제거한다. 단, error/warn 줄과 타임스탬프는 보존한다. 15개 테스트 케이스 평균 82.5% 절약.

웹 페이지 추출은 `contextzip web https://docs.example.com`으로 페이지를 가져와서 nav, footer, 사이드바, 쿠키 배너, 광고, 스크립트를 제거한다. 본문 콘텐츠, 코드 블록, 테이블만 남긴다.


## 3주차: 조건부 필터 + 정직한 벤치마크

필터 3개를 더 만들었다. 빌드 에러 그룹화(TS2322 에러 40개가 하나의 그룹으로, 줄 번호는 전부 보존), 패키지 설치 로그 압축(npm deprecated 노이즈 제거, 보안 경고 보존), Docker 빌드 로그 압축(성공 = 1줄, 실패 = 컨텍스트 보존).

그리고 102개 벤치마크 테스트를 돌렸다. 결과는 솔직했다.

Docker 빌드 로그가 88.2%로 가장 높았고, ANSI/스피너가 82.5%, 에러 스택트레이스가 58.7%였다. 전체 가중 평균은 61.1% — 326K 문자 입력에서 127K 문자 출력.

일부 필터는 작은 입력에서 음수 savings를 보였다. 포맷 오버헤드가 노이즈보다 컸다. README에 그대로 넣었다. 벤치마크를 조작하는 것보다 불완전한 숫자가 낫다.


## Claude Code가 실제로 한 일

모든 작업은 이 사이클을 따랐다. 서브에이전트에 상세 프롬프트를 주고, 에이전트가 기능 + 테스트를 구현하고, 리뷰어 에이전트를 디스패치하고, 이슈를 수정하고, 다음 태스크.

독립적인 태스크는 최대 3개 에이전트를 병렬로 돌렸다. 각 에이전트는 깨끗한 컨텍스트를 받았다 — 이전 작업의 오염이 없다.

최종 결과: 1,056개 테스트 0 실패, 40개 이상의 커맨드 모듈, 5개 플랫폼 CI/CD, Homebrew tap, 4개 언어 README, 102개 벤치마크 테스트.

모든 커밋을 리뷰했다. 아키텍처 결정을 내렸다. 에이전트가 놓친 버그를 잡았다 — Rust panic 압축이 2%에서 시작해서, 내가 함수를 재작성하고 80%까지 올렸다. 하지만 실제 Rust 코드는 Claude가 썼다.


## 써보기

```bash
curl -fsSL https://raw.githubusercontent.com/jee599/contextzip/main/install.sh | bash
```

Claude Code 재시작하면 끝이다. 모든 명령어가 자동으로 압축된다.

매 명령어 뒤에 이런 줄이 뜬다:

```
💾 contextzip: 200 → 40 tokens (saved 80%)
```

전체 절약량 확인: `contextzip gain`

---

- [GitHub: jee599/contextzip](https://github.com/jee599/contextzip)
- [벤치마크 결과 (102개 테스트)](https://github.com/jee599/contextzip/blob/main/docs/benchmark-results.md)
- [RTK](https://github.com/rtk-ai/rtk) 기반

> 가장 좋은 AI 도구는, AI가 더 잘 작동하게 만드는 도구다. 그걸 AI가 만들었더라도.
