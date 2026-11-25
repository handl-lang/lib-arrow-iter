# lib-arrow-iter

`arrow_iter` — arrow iteration over sequences (Handl stdlib port,
pure handl). Single-module package.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the library (pure handl) |
| `pkg.hnl` | package manifest — `modules: ["arrow_iter"]` |
| `scripts/hnl-gate.sh` | the gate — `hnl typecheck`/`lint`/`test` over `src/**/*.hnl` |
| `workflows/gate.yaml` | CI — shared `handl-gates` campaign (bupha/peregrin) |

## Contract

- Pure `.hnl`, no Rust crate; vendored by consumers via
  `handl-pkg-vendor`.
- Provides the iteration arrows other stdlib modules compose over
  sequences.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
