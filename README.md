# host-reference-ocr-helper

The out-of-process OCR helper for
[`host-reference`](https://github.com/connollydavid/host-reference). It recognises the text in an
image and prints it to stdout. It is a separate program by design: the licence-encumbered OCR engine
and its models stay behind an arms-length boundary (host decisions `call/0033` and `call/0034`), so
`host-reference`'s permissive `ocr` plugin runs this binary as a separate process and reads the text
back. That is an aggregation, not a linkage.

## Licence and attribution

This repository carries two licences, by file:

- **Code** (`src/`, `tests/`): **Unlicense** (public domain). See [`LICENSE`](LICENSE).
- **Models** (`models/text-detection.rten`, `models/text-recognition.rten`): **CC-BY-SA-4.0**
  (Creative Commons Attribution-ShareAlike 4.0), the ocrs models by Robert Knight, redistributed
  unmodified with attribution. See [`NOTICE.md`](NOTICE.md) and [`models/NOTICE.txt`](models/NOTICE.txt).

The models are embedded in the built binary. They are content data rather than a linked library, so
including them alongside the public-domain code is an aggregation; the ShareAlike term binds
adaptations of the models, not this code.

## Citations

- ocrs OCR engine, Robert Knight: <https://github.com/robertknight/ocrs> (MIT OR Apache-2.0).
- rten inference runtime, Robert Knight: <https://github.com/robertknight/rten> (MIT OR Apache-2.0).
- ocrs models, Robert Knight: <https://github.com/robertknight/ocrs-models> and
  <https://huggingface.co/robertknight/ocrs> (CC-BY-SA-4.0).

## Use

    host-reference-ocr-helper <image-path>

It prints the recognised text to stdout. The `host-reference` `ocr` plugin finds this binary through
the `HOST_REFERENCE_OCR_HELPER` environment variable or the binary name on `PATH`.

## Build and test

    cargo build --release
    cargo test          # a byte-for-byte conformance golden over a synthetic text image
    cargo deny check    # the dependency-licence lane

The models are embedded, so the built helper is self-contained.
