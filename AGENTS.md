# Workspace-wide Guidance

- Review any project-level `AGENTS.md`, `LLM_Ops_Briefing.md`, or `CLAUDE.md` files that Codex discovers; they contain task-specific guardrails.
- When preparing another LLM, assemble or refresh the matching **LLM Ops Briefing** and store it in the repo docs so Codex and peers stay in sync.
- Prefer language-appropriate, compiled implementations for performance-sensitive scripts; keep legacy Bash helpers as backups until validation is complete.
- Log major tooling or workflow changes in the relevant documentation so future agents inherit the context.

---

## Pre-Flight Checklist

1. Always review project-root guidance files:
   - `CHANGELOG.md`
   - `CONTRIBUTING.md`
   - `.github/` workflows & templates
   - `docs/*guidelines*.md`
2. Follow existing conventions in the codebase before introducing new patterns.

## Git & Version Control

- Do not use Git unless explicitly requested.
- When Git is required:
  - **For major changes: create a backup branch first.**
    ```bash
    git checkout -b backup/before-major-change
    git checkout master
    git checkout -b feature/<work-topic>
    ```
    *Major* umfasst große Refactorings, Architekturänderungen, Breaking APIs, neue Format-Implementierungen – alles, was bestehende Funktionalität gefährden könnte.
  - Danach auf Feature-Branches arbeiten (`feature/...`, `fix/...`, `docs/...`, `refactor/...`).
  - **Ein Commit pro Datei** – jede Datei einzeln stagen/committen, klare Gegenwartsform verwenden.
  - `CHANGELOG.md` bei jeder verhaltensrelevanten Änderung aktualisieren.

## Code Style & Quality

- **No emojis** in code, docs, comments, or commit messages.
- Before committing, run:
  ```bash
  cargo test --workspace
  cargo clippy --all-targets --all-features -- -D warnings
  cargo fmt --all
  ```
- Prefer idiomatic Rust (iterators over index loops, heed clippy unless a justification is documented).
- Keep functions small and focused; document complex logic inline.

## System Configuration

- Sudo commands must use the askpass helper at `/home/pl/.local/share/scripts/system/.secrets/system.askpass.sh` (export `SUDO_ASKPASS` & use `sudo -A`).

## Documentation Expectations

- Discovery order: `CHANGELOG.md` → `CONTRIBUTING.md` → `ARCHITECTURE.md` (if any) → `.github/` → `docs/`.
- Documentation must be clear, concise, and emoji-free; include code examples when useful.

## Package Management

- On Arch-based systems use `paru` first, `pacman` only as fallback.
