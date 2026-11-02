import {
  createScene,
  addSphere,
  sceneToml,
  presets,
  DEFAULT_PRESET,
  MAX_SPHERES,
} from './scene-model.js';

export function mountSceneEditor(onChange, root = document) {
  let model = createScene();
  let selected = 0;
  const visual = root.querySelector('#visual-controls');
  const raw = root.querySelector('#scene');
  const manual = root.querySelector('#manual-toml');
  const select = root.querySelector('#object');
  const remove = root.querySelector('#remove-object');
  const add = root.querySelector('#add-object');
  const presetSelect = root.querySelector('#preset');
  const description = root.querySelector('#preset-description');
  const groups = new Map();
  presetSelect.replaceChildren();
  for (const [key, preset] of Object.entries(presets)) {
    const groupName = preset.group ?? 'Свет и материалы';
    if (!groups.has(groupName)) {
      const group = document.createElement('optgroup');
      group.label = groupName;
      groups.set(groupName, group);
      presetSelect.append(group);
    }
    const option = document.createElement('option');
    option.value = key;
    option.textContent = preset.label;
    groups.get(groupName).append(option);
  }
  presetSelect.value = DEFAULT_PRESET;
  function refresh() {
    description.textContent =
      presets[presetSelect.value].description ??
      'Изменяй сферы, камеру и свет — все настройки доступны ниже.';
    select.replaceChildren(
      ...model.objects.map((o, i) => {
        const option = document.createElement('option');
        option.value = i;
        option.textContent = `Сфера ${i + 1}`;
        return option;
      }),
    );
    select.value = String(selected);
    remove.disabled = model.objects.length === 1;
    add.disabled = model.objects.length >= MAX_SPHERES;
    root.querySelectorAll('[data-object]').forEach((input) => {
      input.value = model.objects[selected][input.dataset.object];
    });
    root.querySelectorAll('[data-scene]').forEach((input) => {
      if (input.type === 'checkbox') input.checked = model[input.dataset.scene];
      else input.value = model[input.dataset.scene];
    });
    root.querySelectorAll('input[type="range"]').forEach((input) => {
      root.querySelector(`output[for="${input.id}"]`).value = input.value;
    });
    raw.value = sceneToml(model);
  }
  function changed() {
    raw.value = sceneToml(model);
    onChange();
  }
  select.addEventListener('change', () => {
    selected = Number(select.value);
    refresh();
  });
  add.addEventListener('click', () => {
    if (addSphere(model)) {
      selected = model.objects.length - 1;
      refresh();
      onChange();
    }
  });
  remove.addEventListener('click', () => {
    if (model.objects.length === 1) return;
    model.objects.splice(selected, 1);
    selected = Math.min(selected, model.objects.length - 1);
    refresh();
    onChange();
  });
  presetSelect.addEventListener('change', (event) => {
    model = createScene(event.target.value);
    selected = 0;
    refresh();
    onChange();
  });
  root.querySelectorAll('[data-object], [data-scene]').forEach((input) => {
    input.addEventListener('input', () => {
      const key = input.dataset.object || input.dataset.scene;
      const value =
        input.type === 'checkbox'
          ? input.checked
          : input.type === 'color'
            ? input.value
            : Number(input.value);
      (input.dataset.object ? model.objects[selected] : model)[key] = value;
      if (input.type === 'range')
        root.querySelector(`output[for="${input.id}"]`).value = input.value;
      changed();
    });
  });
  manual.addEventListener('change', () => {
    visual.disabled = manual.checked;
    raw.readOnly = !manual.checked;
    if (!manual.checked) raw.value = sceneToml(model);
    onChange();
  });
  raw.addEventListener('input', () => {
    if (manual.checked) onChange();
  });
  refresh();
  return { source: () => raw.value };
}
