import assert from 'node:assert/strict';

const base = process.env.PREVIEW_URL || 'http://127.0.0.1:8000';
for (const file of [
  'healthz',
  'index.html',
  'app.js',
  'worker.js',
  'progressive.js',
  'scene-editor.js',
  'scene-model.js',
  'scene-presets.js',
  'image-viewer.js',
  'telegram.js',
  'pkg/raytracer.js',
  'pkg/raytracer_bg.wasm',
]) {
  const response = await fetch(`${base}/${file}`);
  assert.equal(response.status, 200, file);
  assert.equal(response.headers.get('x-content-type-options'), 'nosniff', file);
  assert.equal(response.headers.get('cache-control'), 'no-cache', file);
  if (file.endsWith('.wasm')) {
    assert.equal(response.headers.get('content-type'), 'application/wasm');
    const bytes = new Uint8Array(await response.arrayBuffer());
    assert.equal(WebAssembly.validate(bytes), true);
  } else if (file.endsWith('.js')) {
    assert.match(response.headers.get('content-type'), /(?:application|text)\/javascript/);
  } else if (file === 'healthz') {
    assert.equal(await response.text(), 'ok\n');
  }
}
assert.equal((await fetch(`${base}/missing-file.js`)).status, 404);
console.log(
  'Container HTTP passed: health, frontend assets, MIME types, valid WASM, cache headers and 404',
);
