import { presets } from './scene-presets.js';
export { presets } from './scene-presets.js';

export const DEFAULT_PRESET = 'orbit';
export const MAX_SPHERES = 8;

export function createScene(preset = DEFAULT_PRESET) {
  if (!presets[preset]) throw new Error('Unknown preset');
  const source = presets[preset];
  return {
    objects: source.objects.map((o) => ({ ...o })),
    floor: source.floor ?? true,
    floorColor: source.floorColor ?? '#b4bfce',
    floorChecker: source.floorChecker ?? true,
    checkerColor: source.checkerColor ?? '#5a6b84',
    floorReflection: source.floorReflection ?? 0.32,
    cameraAngle: source.cameraAngle ?? 0,
    cameraHeight: source.cameraHeight ?? 12,
    cameraDistance: source.cameraDistance ?? 20,
    lightAngle: source.lightAngle ?? -35,
    lightHeight: source.lightHeight ?? 8,
    lightPower: source.lightPower ?? 100,
    exposure: source.exposure ?? 1.6,
    ambient: source.ambient ?? 0.35,
    background: source.background ?? '#1e2b50',
    horizon: source.horizon ?? null,
    keyColor: source.keyColor ?? '#ffe0c0',
    rimColor: source.rimColor ?? '#6ebeff',
    fillColor: source.fillColor ?? '#d2b9ff',
    rimPower: source.rimPower ?? 85,
    fillPower: source.fillPower ?? 35,
  };
}
export function addSphere(model) {
  if (model.objects.length >= MAX_SPHERES) return false;
  const colors = ['#ff727e', '#7dceaa', '#82aaff', '#ffb857', '#c5a0ff'];
  model.objects.push({
    x: 0,
    y: 0,
    z: 2,
    radius: 1,
    color: colors[model.objects.length % colors.length],
    reflection: 0.35,
    diffuse: 0,
  });
  return true;
}
function rgb(hex) {
  if (!/^#[0-9a-f]{6}$/i.test(hex)) throw new Error('Invalid colour');
  return [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
}
function numeric(value, min, max) {
  if (!Number.isFinite(value) || value < min || value > max)
    throw new Error('Scene value is outside its supported range');
  return value;
}
const vector = (values) => `[${values.map((v) => Number(v.toFixed(5))).join(', ')}]`;
export function sceneToml(model) {
  if (!model.objects.length || model.objects.length > MAX_SPHERES)
    throw new Error('Use 1–8 spheres');
  const angle = (numeric(model.cameraAngle, -160, 160) * Math.PI) / 180;
  const elevation = (numeric(model.cameraHeight, -45, 60) * Math.PI) / 180;
  const distance = numeric(model.cameraDistance, 12, 35);
  const origin = [
    Math.sin(angle) * Math.cos(elevation) * distance,
    Math.sin(elevation) * distance,
    Math.cos(angle) * Math.cos(elevation) * distance,
  ];
  const lightAngle = (numeric(model.lightAngle, -180, 180) * Math.PI) / 180;
  const light = [
    Math.sin(lightAngle) * 7,
    numeric(model.lightHeight, 1, 12),
    Math.cos(lightAngle) * 7,
  ];
  let text = `[image]
name = "scene"
width = 1024
height = 1024

[camera]
origin = ${vector(origin)}
view = ${vector(origin.map((v) => -v))}
up = [0, 1, 0]

[scene]
exposure = ${numeric(model.exposure, 0.3, 4)}
ambient = ${numeric(model.ambient, 0, 2)}
background = [${rgb(model.background).join(', ')}]
${
  model.horizon
    ? `horizon = [${rgb(model.horizon).join(', ')}]
`
    : ''
}
[[scene.lights]]
origin = ${vector(light)}
power = ${numeric(model.lightPower, 0, 250)}
color = [${rgb(model.keyColor).join(', ')}]

[[scene.lights]]
origin = [6, 5, -4]
power = ${numeric(model.rimPower, 0, 250)}
color = [${rgb(model.rimColor).join(', ')}]

[[scene.lights]]
origin = [-5, 3, 5]
power = ${numeric(model.fillPower, 0, 250)}
color = [${rgb(model.fillColor).join(', ')}]
`;
  for (const obj of model.objects) {
    const center = [obj.x, obj.y, obj.z].map((v) => numeric(v, -6, 6));
    text += `
[[scene.spheres]]
center = ${vector(center)}
radius = ${numeric(obj.radius, 0.3, 3)}
[scene.spheres.properties]
color = [${rgb(obj.color).join(', ')}]
reflection = ${numeric(obj.reflection, 0, 1)}
diffuse = ${numeric(obj.diffuse, 0, 0.8)}
`;
  }
  if (model.floor)
    text += `
[[scene.spheres]]
center = [0, -1002.2, 0]
radius = 1000
[scene.spheres.properties]
color = [${rgb(model.floorColor).join(', ')}]
reflection = ${numeric(model.floorReflection, 0, 1)}
diffuse = 0
${
  model.floorChecker
    ? `checker = [${rgb(model.checkerColor).join(', ')}]
checker_scale = 2
`
    : ''
}
`;
  return text;
}
