# HMSDK-Rust 이어받기 작업 계획

## 📋 현재 상태 (Claude Code → Antigravity 인계)
- **브랜치**: `feature/rust-hmalloc` (18 커밋)
- **최종 커밋**: `bb9b864` — 통합 테스트 링크 수정 (rlib 추가)
- **macOS에서 `cargo check --tests` 통과 확인됨**
- **Linux Docker에서 빌드 시도 → 링크 에러 발생 → 수정 후 커밋 직후 세션 한도 초과**

## ❌ 남은 작업

### Phase 1: Linux 런타임 검증
- [x] Docker Linux 환경에서 `cargo build -p libhmalloc` 성공 확인
- [x] Docker Linux 환경에서 `cargo test -p libhmalloc` 13개 테스트 통과 확인
- [x] Docker Linux 환경에서 `cargo build -p hmctl` 성공 확인
- [x] `HMALLOC_JEMALLOC=1 cargo test -p libhmalloc` (jemalloc 백엔드) 통과 확인
      -> jemalloc `create_arena()` 과정에서 발생하는 OnceCell 교착상태(Deadlock) 버그 해결 완료.

### Phase 2: GitHub Actions CI 추가
- [x] `.github/workflows/ci.yml` 작성
- [x] Linux 러너 + `apt install libjemalloc-dev libnuma-dev`
- [x] `cargo check`, `cargo test`, `cargo clippy` 단계
- [x] jemalloc 백엔드 별도 테스트 단계

### Phase 3: NUMA 멀티노드 통합 테스트 (optional)
- [ ] 실제 NUMA 2+ 노드 환경에서 mbind 정책 검증
- [ ] (하드웨어 의존 — CI에서는 스킵 가능)

## 🔍 참고
- macOS에서는 빌드 불가 (libjemalloc/libnuma 의존), `cargo check`까지만 가능
- `.cargo/config.toml`이 Linux 전용 링커 플래그 분기
- `tikv-jemalloc-sys = "0.5"`가 jemalloc 소스 빌드 포함
