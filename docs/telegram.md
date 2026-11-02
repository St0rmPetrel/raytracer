# Run Raytracer as a Telegram Mini App

The browser renderer is also a static Telegram Main Mini App. All computation
happens in Rust/WASM in a Web Worker on the user's device. No rendering backend,
Telegram webhook, wallet, login server or bot token in frontend code is needed.
The app does not use Telegram user identity, transmit initData or store accounts.

## Prepare the static files

Follow the WASM build commands in README.md. The complete deployment directory is
`web/`, **including generated `web/pkg/` files**. CI also uploads this directory
as the `raytracer-mini-app` artifact. Uploading an artifact does not publish a site.

Serve the directory at a public HTTPS URL using any static host. Keep relative
paths intact (including worker.js, progressive.js, telegram.js, scene-editor.js,
scene-model.js, image-viewer.js and pkg). Serve .wasm as
`application/wasm` and .js files with a JavaScript MIME type. If you configure CSP,
allow the Telegram SDK from https://telegram.org, same-origin workers and WASM
execution. Do not add COEP/COOP headers just for this renderer: it uses a worker,
not WASM threads or SharedArrayBuffer.

## Connect in Telegram

1. Create/select a bot in @BotFather.
2. In the bot's settings, configure its Main Mini App URL to the HTTPS URL above.
3. Open the app from the bot profile's launch button. The main-app link is
   `https://t.me/<your_bot_username>?startapp`.

The app expands on launch, follows Telegram's theme and uses its native bottom
button for rendering. The page button is hidden once the native button is available, leaving one bottom
action. During rendering it becomes an enabled Cancel button with progress; supported
clients ask before closing. Resolution and sampling remain in the scrollable settings.
If native button setup fails, the fixed page button provides the same actions. The visual editor offers presets, sphere
controls, camera/light sliders and a debounced low-resolution auto-preview. TOML
is optional under advanced settings. Success/error haptics are
requested where supported. Final rendering defaults to 1024×1024 / 16 samples in Telegram too. Each render first displays a 32×32 preview and
refines it through progressively larger passes. Cancellation preserves the latest
preview; completion feedback happens only after the final pass. Safe-area insets keep controls out of host chrome.

Open in a normal browser to use the same renderer without Telegram features.

## Verification

```sh
node --test tests/*.mjs
node scripts/check-wasm.mjs
```

The first command checks the host bridge and progressive-frame protocol. The second
executes the real generated WASM artifact. Neither substitutes for a live test on
a phone: check launch, light/dark theme changes, presets, sliders, manual TOML mode,
rendering, cancellation, errors
and reopening. A public deployment and a bot configured through BotFather are
needed for that check. Do not share bot tokens in chat or commit them.
