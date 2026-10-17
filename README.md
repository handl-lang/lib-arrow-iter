# lib-arrow-iter

`arrow_iter` — Handl stdlib port (arrow iteration over sequences).

Part of the stdlib → `lib-*` split (ADR-0128). History is a `git subtree split`
of `stdlib/handl-packages/arrow_iter/`.

## Layout

```text
src/index.hnl   the library (pure handl)
src/lib.rs      const-only crate
pkg.hnl         { modules: ["arrow_iter"] }
ci.sh           bupha entry
```
