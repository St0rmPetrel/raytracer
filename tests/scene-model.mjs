import test from 'node:test';
import assert from 'node:assert/strict';
import { createScene, addSphere, sceneToml, presets } from '../web/scene-model.js';

test('presets are independent and serialize objects, lights and floor', () => {
  for (const name of Object.keys(presets)) {
    const model = createScene(name);
    const toml = sceneToml(model);
    assert.equal(
      (toml.match(/\[\[scene.spheres\]\]/g) || []).length,
      model.objects.length + Number(model.floor),
    );
    assert.equal((toml.match(/\[\[scene.lights\]\]/g) || []).length, 3);
    const fresh = createScene(name);
    model.objects[0].x = 5;
    assert.notEqual(fresh.objects[0].x, 5);
  }
});
test('edits are reflected in generated TOML and floor can be disabled', () => {
  const model = createScene();
  model.objects[0].color = '#123456';
  model.objects[0].radius = 2.5;
  model.floor = false;
  const toml = sceneToml(model);
  assert.ok(toml.includes('color = [18, 52, 86]'));
  assert.ok(toml.includes('radius = 2.5'));
  assert.ok(!toml.includes('-1002.2'));
  const original = toml;
  model.cameraAngle = 90;
  model.lightAngle = -90;
  assert.notEqual(sceneToml(model), original);
});
test('sphere count is bounded and unsupported values fail early', () => {
  const model = createScene();
  while (addSphere(model)) {}
  assert.equal(model.objects.length, 8);
  model.objects[0].radius = NaN;
  assert.throws(() => sceneToml(model));
  model.objects[0].radius = 1;
  model.objects[0].color = 'bad';
  assert.throws(() => sceneToml(model));
  assert.throws(() => createScene('missing'));
});

test('orbit opens with eight spheres, studio lighting and configurable exposure', () => {
  const model = createScene();
  assert.equal(model.objects.length, 8);
  assert.equal(model.cameraDistance, 17);
  model.exposure = 2.5;
  model.lightPower = 180;
  const toml = sceneToml(model);
  assert.ok(toml.includes('exposure = 2.5'));
  assert.ok(toml.includes('power = 180'));
  assert.ok(toml.includes('checker_scale = 2'));
  model.exposure = NaN;
  assert.throws(() => sceneToml(model));
  model.exposure = 1;
  model.lightPower = -1;
  assert.throws(() => sceneToml(model));
});
