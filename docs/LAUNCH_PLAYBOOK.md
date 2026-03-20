# GitHub 1,000+ Stars in First Week: Launch Playbook

> 실전 리서치 기반. 2026년 3월 기준.
> Daytona (4,000 stars/1주), The Agency (10,000 stars/7일), Novu (2K->20K/1년), ScrapeGraphAI, Lago 등 실제 사례 분석.

---

## TL;DR: 핵심 공식

```
좋은 README + Hacker News 1페이지 + Twitter 바이럴 스레드 + Reddit 크로스포스팅
= 1주일 내 1,000+ stars 가능
```

HN 1페이지 단독으로 평균 289 stars/7일 (바이럴 시 1,200 stars/24시간).
Twitter 스레드 단독으로 800 stars/주.
두 채널 동시 히트 시 시너지 발생.

---

## Phase 0: 런칭 전 준비 (T-2주 ~ T-1일)

### 0-1. README 최적화 (필수, 1회성)

Daytona가 4,000 stars를 1주에 달성한 핵심은 README 퀄리티였다.

**필수 요소:**
- 로고 + 뱃지 (build status, license, version)
- 30초 데모 GIF (핵심 가치를 즉시 보여줌)
- 원라이너 설명 + 컨텍스트 서브타이틀
- Quick Start 가이드 (3줄 이내 설치 + 실행)
- Why 섹션 (이 프로젝트가 왜 필요한지)
- Feature 리스트 (bullet points)
- Contributing 가이드, License, Code of Conduct
- "Good first issue" 라벨이 붙은 이슈 5-10개

**자동화 가능 여부:** 부분적. 데모 GIF 녹화는 수동이지만, 뱃지 생성, 이슈 템플릿 설정 등은 스크립트화 가능.

**예상 임팩트:** 직접 star를 만들진 않지만, 모든 채널에서 유입된 트래픽의 전환율을 2-3배 높임.

---

### 0-2. 소셜 계정 준비

**Hacker News:**
- 런칭 1주 전부터 5-7개 스토리에 코멘트 달아서 20+ 카르마 확보
- 250+ 카르마가 이상적이지만, 20+이면 Show HN 가능
- 코멘트 히스토리가 없으면 스팸 필터에 걸림

**Reddit:**
- 타겟 서브레딧에서 2주간 genuine engagement
- r/programming, r/opensource, r/selfhosted, r/commandline 등
- 셀프 프로모션 규칙 확인 (대부분 10:1 비율 요구)

**Twitter/X:**
- 개발자 인플루언서 5-10명 팔로우 + 의미 있는 리플라이
- 런칭 전에 2-3개 기술 트윗으로 가볍게 워밍업

---

## Phase 1: 런칭 당일 (D-Day)

### 1-1. Hacker News "Show HN" (최우선)

**연구 데이터:**
- HN 1페이지 노출 시 평균 121 stars/24시간, 289 stars/7일
- 바이럴 시 1,200 stars/24시간 가능 (ScrapeGraphAI 사례)
- 12-17 UTC (한국시간 21:00-02:00) 포스팅 시 +200 stars 추가 효과
- 화요일-목요일이 최적. 금요일 오후, 월요일 아침 피하기
- 일요일 18:00-21:00 PT도 경쟁 40% 적어서 유리

**포스팅 프로토콜:**
1. 타이틀: 45-65자. 구체적 숫자 + 혜택. 예: "Show HN: TokenZip - Compress LLM context by 70% with zero quality loss"
2. 주거용 IP 또는 모바일 핫스팟에서 제출 (사무실 IP는 카르마 풀 공유)
3. 제출 즉시 60단어 TL;DR 코멘트 + 질문 1개로 토론 유도
4. GitHub 링크를 직접 제출 (브랜드 URL보다 GitHub가 HN에서 유리)
5. 첫 30분 내 8-10 업보트 + 2-3개 실질적 코멘트 필요
6. 모든 코멘트에 10분 내 답변

**자동화 가능 여부:** 제출과 모니터링은 자동화 가능하나, 코멘트 응대는 수동. HN은 인위적 업보트 조작을 감지하므로 organic이어야 함.

**예상 임팩트:** 100-1,200 stars (HN 스코어에 비례)

**리스크:** 업보트 조작 시 shadowban. 공개적으로 업보트 요청하면 페널티. 타이틀 2번 이상 수정하면 타임스탬프 리셋.

---

### 1-2. Twitter/X 바이럴 스레드 (HN과 동시)

**데이터:** Twitter 스레드 단독으로 800 stars/주 가능. The Agency는 Greg Isenberg 한 명의 트윗으로 수천 stars 유입.

**스레드 구조 (5-8 트윗):**
1. 훅: 문제 제시 또는 놀라운 수치 ("LLM API costs eating your budget? I compressed context by 70%.")
2. 문제 공감
3. 해결책 제시 (데모 GIF)
4. 결과 수치
5. 기술 깊이 (짧게)
6. 사용 사례
7. GitHub 링크 + star CTA
8. 리트윗 요청

**자동화:** Hypefury 또는 Postiz로 스레드 예약 가능. 최적 시간: 화-목 8-10 AM PT.

**예상 임팩트:** 200-800 stars/주

**리스크:** 낮음. 단, 과도한 셀프 프로모션은 engagement 하락.

---

### 1-3. Reddit 크로스포스팅 (D-Day 또는 D+1)

**데이터:** Reddit 바이럴 시 725K views, 5.7K upvotes, 1,500 signups 사례 존재.

**타겟 서브레딧:**
- r/programming (5M+ 멤버)
- r/opensource
- r/commandline
- r/selfhosted
- r/artificial (AI 관련이면)
- r/LocalLLaMA (LLM 관련이면)

**포스팅 규칙:**
- 판매 냄새 내지 말 것. "war story" 또는 "lesson learned" 포맷
- 제목: 호기심 유발, 100자 이내, 구체적 숫자
- 7-9 AM EST 또는 5-7 PM EST 포스팅
- 모든 코멘트에 즉각 응대, 실시간 버그 수정
- 친구에게 업보트 요청하지 않아도 됨 (organic이 더 효과적)

**자동화:** 포스팅 자체는 API로 가능하나, 각 서브레딧 규칙 준수를 위해 수동 커스터마이즈 권장.

**예상 임팩트:** 50-500 stars (서브레딧과 바이럴 정도에 따라)

**리스크:** 셀프 프로모션 규칙 위반 시 ban. 10:1 비율(일반 참여:프로모션) 유지.

---

## Phase 2: D+1 ~ D+3 (모멘텀 유지)

### 2-1. DEV.to / Hashnode / HackerNoon 기술 포스트

**데이터:** 기술 블로그 포스트 단독으로 500 stars/일 가능.

**콘텐츠 4가지 유형:**
1. **직접 소개:** "TokenZip 소개: LLM 컨텍스트를 70% 압축하는 방법"
2. **간접 튜토리얼:** "Claude API 비용을 절반으로 줄이는 5가지 방법" (그중 하나로 TokenZip 언급)
3. **리스트클:** "2026년 LLM 개발자를 위한 필수 도구 10선"
4. **빌드 인 퍼블릭:** "주말 프로젝트가 1주 만에 1,000 stars를 달성한 이야기"

**크로스포스팅 자동화:**
- canonical URL을 원본 블로그로 설정
- DEV.to API, Hashnode API로 자동 syndication
- GitHub Actions 또는 Postiz로 자동화 가능
- 도달 범위 300-500% 증가 (draft.dev 데이터)

**자동화 가능 여부:** 높음. 글 작성 후 크로스포스팅은 완전 자동화 가능. `cross-post` CLI 도구, Postiz, GitHub Actions 활용.

**예상 임팩트:** 100-500 stars/포스트

---

### 2-2. GitHub Trending 진입

**알고리즘 핵심:**
- 절대 star 수가 아니라 **star velocity** (증가율 대비 평균)가 중요
- 평소 2 stars/일 받다가 10 stars/일 받으면 → 평소 50/일 받다가 60/일보다 트렌딩 점수 높음
- 1-2시간 내 30-40 stars 집중되면 트렌딩 진입 가능성 높음
- forks, issues, PRs, comments 등 전반적 활동도 가중치
- 니치 언어일수록 트렌딩 임계값 낮음 (Rust는 JS보다 적은 stars로 트렌딩)

**전략:**
- Phase 1의 모든 채널 트래픽을 **같은 2-3시간 윈도우**에 집중
- HN 제출 → 즉시 Twitter 스레드 → 30분 후 Reddit
- 이 집중 트래픽이 트렌딩 알고리즘을 트리거

**자동화:** 포스팅 스케줄링으로 트래픽 집중 자동화 가능.

**예상 임팩트:** 트렌딩 진입 시 추가 500-2,000 stars/일 (셀프 증폭 효과)

---

### 2-3. Awesome Lists PR 제출

**방법:**
- GitHub에서 "awesome-{your-niche}" 검색
- 관련 Awesome List에 PR 제출
- 포맷, 기여 가이드라인 준수
- star 수가 있으면 수락 확률 높음

**자동화:** PR 제출 자체는 수동이지만, 관련 리스트 검색은 스크립트화 가능.

**예상 임팩트:** 리스트당 20-100 stars (리스트 인기도에 따라)

**리스크:** 없음. 정당한 기여.

---

## Phase 3: D+4 ~ D+7 (확장)

### 3-1. Product Hunt 런칭

**데이터:** Daytona는 Product Hunt에서 #2 달성 → 수천 stars 유입. Supabase는 16번 런칭.

**준비:**
- 헌터(Hunter) 확보: 팔로워 많은 유저에게 사전 연락
- 제품 페이지: 5장 이미지/GIF, 명확한 설명, 첫인상이 결정적
- 런칭일: 화-목 00:01 PT
- 첫 1시간 내 모든 코멘트에 응대

**자동화:** 페이지 준비는 수동. 런칭 알림은 자동화 가능.

**예상 임팩트:** 100-500 stars

---

### 3-2. 인플루언서 아웃리치

**The Agency 사례:** Greg Isenberg 1명의 트윗으로 수천 stars.

**전략:**
- AI/개발자 도구 인플루언서 리스트 작성 (20-30명)
- 짧은 DM: 문제 → 솔루션 → 데모 링크 → "피드백 부탁"
- 리스트클 작성 시 다른 프로젝트 태깅 → 상호 리트윗
- 유료 리뷰 또는 스폰서 포스트도 옵션

**자동화:** DM 템플릿 자동화 가능하나, 개인화가 효과적.

**예상 임팩트:** 인플루언서 1명당 50-500 stars

---

### 3-3. 뉴스레터 / 어그리게이터 제출

**채널:**
- GitHub20K (무료 제출)
- Console.dev (무료 또는 스폰서)
- daily.dev (승인 필요)
- TLDR Newsletter
- Changelog
- Hacker Newsletter

**자동화:** 제출 자체는 수동 (1회성, 각 10분).

**예상 임팩트:** 채널당 20-200 stars

---

### 3-4. YouTube Shorts / 데모 영상

**데이터:** 컨퍼런스 토크 영상으로 2,000 stars/월 가능.

**전략:**
- 60초 이내 데모 영상 (문제 → 해결 → 결과)
- 첫 3초가 결정적 (Average Percentage Viewed 100%+ 목표)
- 루핑 가능한 구조
- GitHub 링크를 description과 pinned comment에

**자동화:** 영상 제작은 수동이나, 배포는 Postiz/Hypefury로 자동화.

**예상 임팩트:** 50-500 stars/영상

---

## 자동화 가능한 도구 스택

| 도구 | 용도 | 가격 |
|------|------|------|
| **Postiz** | 17+ 플랫폼 소셜 스케줄링, 크로스포스팅 | 무료 (셀프호스트) |
| **Hypefury** | Twitter 스레드 예약, auto-retweet, 크로스포스팅 | $19/월~ |
| **Buffer** | 소셜 미디어 스케줄링 | $6/채널/월 |
| **cross-post CLI** | DEV.to, Hashnode, Medium 동시 발행 | 무료 |
| **GitHub Actions** | 블로그 → DEV.to/Hashnode 자동 싱크 | 무료 |
| **star-history.com** | star 증가 추적 | 무료 |
| **hn.algolia.com** | HN 트래픽 분석, 최적 시간 탐색 | 무료 |

---

## 전체 타임라인 요약

| 시점 | 액션 | 예상 stars | 자동화 |
|------|------|-----------|--------|
| T-2주 | README 최적화, 소셜 계정 워밍업 | 0 | 부분 |
| T-1주 | 콘텐츠 작성, 스레드 준비, HN 카르마 축적 | 0 | 부분 |
| D-Day AM | HN Show HN 제출 + Twitter 스레드 + Reddit | 200-500 | 스케줄링 |
| D+1 | DEV.to/Hashnode 기술 포스트 발행 | 100-300 | 자동 크로스포스팅 |
| D+2 | GitHub Trending 진입 (자동 증폭) | 200-1,000 | 자동 |
| D+3 | 추가 서브레딧, 인플루언서 아웃리치 | 100-300 | 부분 |
| D+4-5 | Product Hunt 런칭 | 100-500 | 부분 |
| D+6-7 | 뉴스레터/어그리게이터, YouTube | 50-200 | 부분 |
| **합계** | | **750-2,800** | |

---

## 절대 하지 말 것

1. **가짜 stars 구매**: GitHub이 적극 단속 중. 2024년 기준 600만 개 의심 fake stars 탐지 (arXiv 논문). 레포 삭제 또는 ban 리스크.
2. **업보트 봇 사용**: HN, Reddit 모두 조작 감지 알고리즘 보유. 계정 shadowban.
3. **스팸 DM**: 대량 발송은 계정 정지 + 평판 손상.
4. **여러 계정으로 자작 업보트**: IP 기반 탐지에 걸림.
5. **가치 없는 콘텐츠 대량 생산**: 커뮤니티에서 배제됨.

---

## 핵심 원칙

> "Stars without strategy are vanity metrics. Stars with intent are business drivers."

1. **문제를 해결하는 프로젝트가 전제조건이다.** 마케팅은 증폭기일 뿐, 없는 가치를 만들지 않는다.
2. **트래픽을 2-3시간에 집중시켜라.** 분산된 트래픽은 트렌딩을 트리거하지 못한다.
3. **모든 커버리지를 연결해라.** HN 포스트에 Twitter 언급, Twitter에 HN 링크, 블로그에 모든 링크. 플라이휠을 만들어라.
4. **첫 100 stars는 네트워크에서.** 개인 네트워크 60% 응답률. 이 임계값을 넘어야 organic이 작동한다.
5. **화-목 런칭이 최적이다.** 주말과 월요일 피하기.

---

## Sources

- [GitHub Star Growth: Battle-Tested Playbook](https://dev.to/iris1031/github-star-growth-a-battle-tested-open-source-launch-playbook-35a0)
- [Lago: How we got first 1000 stars](https://www.getlago.com/blog/how-we-got-our-first-1000-github-stars)
- [Star History: Playbook for More GitHub Stars](https://www.star-history.com/blog/playbook-for-more-github-stars)
- [Daytona: How to Write a 4000 Stars README](https://www.daytona.io/dotfiles/how-to-write-4000-stars-github-readme-for-your-project)
- [10 Proven Ways to Boost GitHub Stars 2026](https://scrapegraphai.com/blog/gh-stars)
- [HackerNoon: Ultimate Playbook for GitHub Stars](https://hackernoon.com/the-ultimate-playbook-for-getting-more-github-stars)
- [Launch-Day Diffusion: HN Impact on GitHub Stars (arXiv)](https://arxiv.org/html/2511.04453v1)
- [The Agency: 10K Stars in 7 Days](https://www.prismnews.com/topics/agency-seo-growth/ai-repo-the-agency-surpasses-10k-github-stars-in-just-7-days)
- [Flowjam: HN Front Page Playbook 2025](https://www.flowjam.com/blog/how-to-get-on-the-front-page-of-hacker-news-in-2025-the-complete-up-to-date-playbook)
- [Nevo David: GitHub 20K Stars Strategy](https://dev.to/crowddotdev/how-to-grow-a-github-project-to-20k-stars-with-nevo-david-novu-3i81)
- [Gitroom: Open-Source Growth Funnel](https://gitroom.com/blog/current-funnel)
- [Reddit Viral: 725K views, 5.7K upvotes](https://www.indiehackers.com/post/how-to-go-viral-on-reddit-725-000-views-5-7k-upvotes-1-000-signups-d029809634)
- [Show HN Survival Study](https://asof.app/research/show-hn-survival)
- [Cross-Posting Automation](https://dev.to/ryancwynar/cross-posting-automation-publish-once-syndicate-everywhere-32h2)
- [GitHub Trending Algorithm Discussion](https://github.com/orgs/community/discussions/163970)
- [6 Million Fake Stars on GitHub (arXiv)](https://arxiv.org/html/2412.13459v2)
