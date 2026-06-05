# HMSDK-Rust 이어받기 작업 계획

## 📋 현재 상태 (Claude Code → Antigravity 인계)
- **브랜치**: `feature/rust-hmalloc` (18 커밋)
- **최종 커밋**: `bb9b864` — 통합 테스트 링크 수정 (rlib 추가)
- **macOS에서 `cargo check --tests` 통과 확인됨**
- **Linux Docker에서 빌드 시도 → 링크 에러 발생 → 수정 후 커밋 직후 세션 한도 초과**

## 🧪 프로젝트 10단계 테스트 검증 (feature/10-stage-testing)

- [x] **Step 1: 기본 빌드 및 정적 분석 재검증** (`cargo check`, `cargo clippy` 경고 Zero 확인)
- [x] **Step 2: 환경 변수 파싱 및 NUMA 정책 초기화 한계 테스트** (잘못된 환경변수, 엣지 케이스 주입 시 안전성 확인)
- [ ] **Step 3: C ABI FFI 경계 예외 테스트** (`NULL` 포인터 해제, 0바이트 할당, 과도한 크기 할당 시 Segfault 방지)
- [ ] **Step 4: 멀티스레드 동시 할당/해제 스트레스 테스트** (수백 개 스레드 동시 할당 시 스레드 안전성 확인)
- [ ] **Step 5: 대규모 메모리(Large Allocation) 및 단편화 테스트** (기가바이트 단위 할당 및 해제 반복 시 OOM 및 안정성 검증)
- [ ] **Step 6: Jemalloc extent hook 정밀 검증** (커스텀 훅이 예상대로 호출되며 정확한 크기와 정렬로 반환하는지 검증)
- [ ] **Step 7: NUMA `mbind` 노드 바인딩 실제 확인** (BIND, INTERLEAVE 등 정책 적용 후 할당된 메모리 주소의 물리 노드 확인)
- [ ] **Step 8: 런타임 Deadlock 및 Reentrancy 회귀 테스트** (`OnceCell` 초기화 구간 및 백엔드 락(lock) 경쟁 검증)
- [ ] **Step 9: `hmctl` 바이너리를 통한 프로세스 생성 및 `LD_PRELOAD` 환경 주입 E2E 테스트**
- [ ] **Step 10: 메모리 누수 및 UB(Undefined Behavior) 심층 검증** (Valgrind, Miri 또는 ASan을 활용한 정적/동적 메모리 무결성 분석)

## 🔍 참고
- macOS에서는 빌드 불가 (libjemalloc/libnuma 의존), `cargo check`까지만 가능
- `.cargo/config.toml`이 Linux 전용 링커 플래그 분기
- `tikv-jemalloc-sys = "0.5"`가 jemalloc 소스 빌드 포함
