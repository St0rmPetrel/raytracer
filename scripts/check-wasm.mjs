import fs from 'node:fs';
import assert from 'node:assert/strict';
const root = new URL('../', import.meta.url).pathname.replace(/\/$/, '');
const {
  default: init,
  render_scene,
  render_scene_quality,
  BrowserRenderer,
} = await import(`${root}/web/pkg/raytracer.js`);
await init({
  module_or_path: fs.readFileSync(`${root}/web/pkg/raytracer_bg.wasm`),
});
const scene = fs.readFileSync(`${root}/web/scene.toml`, 'utf8');
for (const [w, h] of [
  [1, 1],
  [32, 32],
  [12, 8],
]) {
  const result = render_scene(scene, w, h);
  assert.equal(result.length, w * h * 4);
  assert.ok(result.every((v, i) => i % 4 !== 3 || v === 255));
  if (w === 32) assert.ok(result.some((v, i) => i % 4 !== 3 && v > 0));
}
assert.throws(() => render_scene(scene, 0, 16));
assert.throws(() => render_scene('invalid', 16, 16));
console.log('Real WASM execution passed: square/rectangular RGBA, non-black scene, invalid inputs');

const { renderProgressively } = await import(`${root}/web/progressive.js`);
const frames = [];
await renderProgressively({
  scene,
  size: 128,
  render: render_scene,
  publish: (frame) => {
    assert.equal(frame.rgba.length, frame.size * frame.size * 4);
    frames.push({ size: frame.size, done: frame.done });
  },
});
assert.deepEqual(frames, [
  { size: 32, done: false },
  { size: 64, done: false },
  { size: 128, done: true },
]);
console.log('Real WASM progressive rendering passed: 32 → 64 → 128');

const { createScene, sceneToml, presets, addSphere } = await import(`${root}/web/scene-model.js`);
for (const preset of Object.keys(presets)) {
  const model = createScene(preset);
  for (const rotation of [-160, 0, 160]) {
    model.cameraAngle = rotation;
    const result = render_scene(sceneToml(model), 32, 32);
    assert.equal(result.length, 32 * 32 * 4);
    assert.ok(result.some((v, i) => i % 4 !== 3 && v > 0));
  }
}
const edited = createScene();
edited.floor = false;
edited.cameraHeight = 60;
edited.lightAngle = -180;
edited.objects[0].color = '#123456';
edited.objects[0].radius = 0.3;
while (addSphere(edited)) {}
assert.equal(render_scene(sceneToml(edited), 16, 16).length, 16 * 16 * 4);
console.log('Visual scene presets and edited scenes rendered successfully in real WASM');

for (const samples of [1, 4, 16]) {
  assert.equal(render_scene_quality(sceneToml(createScene()), 32, 32, samples).length, 32 * 32 * 4);
}
for (const samples of [0, 2, 17]) assert.throws(() => render_scene_quality(scene, 32, 32, samples));
console.log('Real WASM quality modes passed: 1 / 4 / 16 samples and invalid quality rejected');

const large = render_scene_quality(sceneToml(createScene()), 1024, 1024, 1);
assert.equal(large.length, 1024 * 1024 * 4);
assert.ok(large.every((v, i) => i % 4 !== 3 || v === 255));
assert.throws(() => render_scene_quality(scene, 1025, 32, 1));
console.log('Real WASM 1024×1024 RGBA passed; 1025 rejected');

const deterministic = sceneToml(createScene());
const incremental = new BrowserRenderer(deterministic, 128, 128, 4);
try {
  const strips = [];
  for (let row = 0; row < 128; row += 8) strips.push(incremental.render_rows(row, 8));
  assert.deepEqual(
    Buffer.concat(strips),
    Buffer.from(render_scene_quality(deterministic, 128, 128, 4)),
  );
  for (const [row, rows] of [
    [0, 0],
    [0, 65],
    [128, 1],
    [127, 2],
  ])
    assert.throws(() => incremental.render_rows(row, rows));
} finally {
  incremental.free();
}
console.log('Real WASM incremental strips match full render; invalid strips rejected');
