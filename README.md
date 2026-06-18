# @camera.ui/rust-decoder

Native Rust pixel-format conversion, resize and crop for the [camera.ui](https://github.com/seydx/camera.ui) ecosystem.

Built with [napi-rs](https://napi.rs) — ships prebuilt binaries for Linux (glibc/musl), macOS, Windows and FreeBSD across x64, arm64 and riscv64, so there is no compile step on install.

## Installation

```bash
npm install @camera.ui/rust-decoder
```

## Usage

```ts
import {
  convertNv12ToRgb,
  processImage,
  resizeImage,
  cropImage,
} from '@camera.ui/rust-decoder';

// NV12 frame -> packed RGB
const rgb = convertNv12ToRgb(frame, width, height);

// One-shot convert + crop + resize pipeline
const out = processImage(
  inputFrame, inputWidth, inputHeight,
  inputFormat, outputFormat,
  cropTop, cropLeft, cropWidth, cropHeight,
  resizeWidth, resizeHeight,
);
```

Also exposes individual `convert*` helpers (NV12/YUV → RGB/RGBA/grayscale,
BGRA/RGBA/RGB conversions), plus `resizeImage`, `cropImage` and `resizeAndCrop`.
See `index.d.ts` for the full signatures.

## Development

```bash
npm install
npm run build        # release build (napi build --platform --release)
npm run build:debug  # debug build
npm run lint         # cargo clippy + eslint
```

---

_Part of the camera.ui ecosystem - A comprehensive camera management solution._
