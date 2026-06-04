# Lessons Learned

## 2026-06-04: Claude Code 세션 로그 인계 패턴
- **트리거**: 유저가 "Claude 리밋 걸렸다" / "Claude 세션 한도" 등 말하면
- **액션**: `~/.claude/projects/<프로젝트경로>/*.jsonl` 파일을 읽어서 Claude Code가 마지막으로 작업한 내용을 파악
- **목적**: Antigravity CLI가 Claude Code의 작업을 이어받아 계속 진행
- **로그 위치**: `~/.claude/projects/-Users-kikicaca44-Desktop-code-hmsdk-rust-hmsdk-rust/`
