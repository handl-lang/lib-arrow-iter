# lib-arrow-iter

`arrow_iter` — Handl stdlib port (arrow iteration over sequences).

## Layout

```text
src/index.hnl            the library (pure handl)
pkg.hnl                  package manifest
scripts/hnl-gate.sh      the gate
workflows/gate.yaml                    bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
