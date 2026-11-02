# Raytracer

A Rust raytracer that runs as a native CLI, in a browser or as a Telegram Mini App.
Browser rendering happens on the device in a Web Worker; no rendering backend is
required. The same Rust library powers both modes.

## Browser app

- 16 editable presets grouped into space, materials and playful compositions.
- Sphere, camera and lighting controls, plus optional manual TOML editing.
- Linear lighting, coloured lights, shadows, reflections and tone mapping.
- 128/256/512/1024 resolution and 1/4/16 primary samples per pixel.
- Progressive resolution and streamed image strips, cancellation and automatic
  low-resolution previews. Tap the image for expanded viewing with live progress.
- Telegram themes, native render button and supported-client haptics.

Scene changes reset on reload. Manual TOML is a separate mode: returning to visual
editing restores the last visual scene. Ambient fill approximates indirect light;
this is a recursive raytracer, not a global-illumination path tracer.

## Native CLI

```sh
cargo run --locked --release -- config/reflect.toml
```

Writes the configured binary PPM image. See [example scene](config/reflect.toml)
for the TOML format. Old scenes still parse; the updated lighting changes their
appearance. Optional settings include image `samples`, scene `exposure`, `ambient`,
`background` and `horizon`, light `power` and `color`, and material `checker` and
`checker_scale`.

## Build and run locally

Use Rust 1.99 or newer. Node 22+ is needed only for tests.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.130 --locked
cargo build --locked --release --lib --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir web/pkg target/wasm32-unknown-unknown/release/raytracer.wasm
python3 -m http.server 8000 --bind 127.0.0.1 --directory web
```

Open <http://127.0.0.1:8000>. Generated `web/pkg/` files are ignored by Git.
The wasm-bindgen CLI must match the version in Cargo.lock. Automatic previews use
at most 128×128 / 1 sample; final rendering defaults to 1024×1024 / 16 samples.
1024×1024 takes roughly four times the work of 512 at the same sample count.

## Docker

```sh
docker build -t raytracer .
docker run --rm --read-only --tmpfs /tmp:rw,noexec,nosuid,size=16m \
  --cap-drop ALL --security-opt no-new-privileges:true \
  -p 127.0.0.1:8000:8080 raytracer
```

Open <http://127.0.0.1:8000>. The multi-stage build compiles WASM from source;
the runtime contains static assets and unprivileged nginx on port 8080, with a
`/healthz` check. Base images are pinned by digest. Build tools and local generated
assets stay out of the runtime image. nginx revalidates unversioned assets to avoid
stale JS/WASM combinations after updates.

For a VDS behind the existing Traefik proxy, see [deployment](deploy/README.md).
For Telegram, serve behind HTTPS and configure the bot's Mini App URL. See
[Telegram setup](docs/telegram.md). No bot token belongs in the frontend.

## Code layout

- `src/`: shared renderer and configuration; `browser.rs` contains the WASM bindings.
- `web/`: browser UI, editor, worker and progressive rendering. Preset data lives in
  `scene-presets.js`; `scene-model.js` creates editable models and serializes TOML.
- `tests/`: JS unit tests; `tests/browser/` contains Playwright tests and configuration,
  including a simulated Telegram host (not a real device test).
- `scripts/`: HTTP and real-WASM integration checks; `deploy/`: Compose setup.

## Verification

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
npm ci
npm test
npm run format:check
node scripts/check-wasm.mjs # after generating bindings
npx playwright install chromium
npm run test:browser # with the local server or container running
npm run test:layout # only Telegram layout checks (also included in test:browser)
node scripts/check-http.mjs # container only
```

Set `CHROME_PATH` to use an installed Chrome instead of downloading Chromium.
`PREVIEW_URL` overrides the test URL (default `http://127.0.0.1:8000`). Browser
checks exercise real WASM on desktop/mobile viewports, all presets, controls,
1024 rendering, cancellation and updates while the image viewer is open. The HTML report,
failure screenshots and traces go to ignored `review-output/` (open the report
with `npx playwright show-report review-output/report`). Each scenario runs in an
isolated browser context.

CI runs native/WASM checks, builds the Docker image and tests the app served from
that image. It uploads artifacts without deploying a site or publishing an image.
Live Telegram/phone testing still requires a configured bot and HTTPS hosting.
