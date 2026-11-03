import { connectTelegram } from './telegram.js';
import { mountSceneEditor } from './scene-editor.js';
import { mountImageViewer } from './image-viewer.js';

const renderButton = document.querySelector('#render');
const cancelButton = document.querySelector('#cancel');
const statusLabel = document.querySelector('#status');
const canvas = document.querySelector('#scene-canvas');
mountImageViewer(canvas);
const stripCanvas = document.createElement('canvas');
const livePreview = document.querySelector('#live-preview');
let worker;
let previewTimer;
let ready = false;
const telegram = connectTelegram({
  onRender: () => render(false),
  onCancel: cancelRender,
});
document.body.classList.toggle('telegram-native', telegram.nativeButton);
function stopRendering() {
  worker?.terminate();
  worker = undefined;
  renderButton.disabled = !ready;
  cancelButton.hidden = true;
  document.body.classList.remove('rendering');
  telegram.setState({ ready, busy: false });
}
function clearScheduledPreview() {
  clearTimeout(previewTimer);
  previewTimer = undefined;
}
function fail(message, quiet = false) {
  statusLabel.textContent = message;
  if (!quiet) telegram.feedback(false);
  stopRendering();
}

function drawFrame(frame) {
  const rows = frame.rows ?? frame.size;
  const row = frame.row ?? 0;
  stripCanvas.width = frame.size;
  stripCanvas.height = rows;
  stripCanvas
    .getContext('2d')
    .putImageData(new ImageData(new Uint8ClampedArray(frame.rgba), frame.size, rows), 0, 0);
  if (canvas.width !== frame.targetSize) {
    const previous = document.createElement('canvas');
    previous.width = canvas.width;
    previous.height = canvas.height;
    previous.getContext('2d').drawImage(canvas, 0, 0);
    canvas.width = canvas.height = frame.targetSize;
    canvas.getContext('2d').drawImage(previous, 0, 0, canvas.width, canvas.height);
  }
  const context = canvas.getContext('2d');
  context.imageSmoothingEnabled = false;
  const scale = canvas.width / frame.size;
  context.drawImage(
    stripCanvas,
    0,
    0,
    frame.size,
    rows,
    0,
    row * scale,
    canvas.width,
    rows * scale,
  );
  canvas.dataset.renderSize = frame.size;
  canvas.dataset.renderRow = row + rows;
  canvas.dataset.frame = Number(canvas.dataset.frame || 0) + 1;
}

function render(isPreview = false) {
  if (!ready) return;
  clearScheduledPreview();
  stopRendering();
  const requested = Number(document.querySelector('#size').value);
  const samples = isPreview ? 1 : Number(document.querySelector('#quality').value);
  const size = isPreview ? Math.min(128, requested) : requested;
  renderButton.disabled = true;
  cancelButton.hidden = false;
  document.body.classList.add('rendering');
  telegram.setState({ ready, busy: true });
  statusLabel.textContent = 'Проявляем сцену…';
  try {
    const activeWorker = new Worker('./worker.js', { type: 'module' });
    worker = activeWorker;
    worker.onmessage = ({ data }) => {
      if (worker !== activeWorker) return;
      if (data.error) {
        fail(`Не удалось построить сцену: ${data.error}`, isPreview);
        return;
      }
      try {
        drawFrame(data);
        if (data.done) {
          statusLabel.textContent = `${isPreview ? 'Превью' : 'Готово'} · ${data.size} × ${data.size} · ${samples} ${samples === 1 ? 'луч/пиксель' : 'лучей/пиксель'} · ${(data.elapsed / 1000).toFixed(2)} с${isPreview ? '. Нажми «Перерисовать» для итогового качества.' : ''}`;
          if (!isPreview) telegram.feedback(true);
          stopRendering();
        } else {
          const renderedRows = (data.row ?? 0) + (data.rows ?? data.size);
          statusLabel.textContent = `Уточняем: ${data.size} × ${data.size} → ${data.targetSize} × ${data.targetSize} · проход ${data.pass}/${data.passes} · ${Math.round((renderedRows / data.size) * 100)}%`;
        }
      } catch {
        fail('Не удалось показать изображение. Попробуй меньшее разрешение.', isPreview);
      }
    };
    worker.onerror = () => {
      if (worker === activeWorker)
        fail('Не удалось загрузить рендерер. Проверь сборку WASM и хостинг.', isPreview);
    };
    worker.postMessage({ scene: editor.source(), size, samples });
  } catch {
    fail('Не удалось запустить рендерер в этом браузере.', isPreview);
  }
}
function schedulePreview() {
  clearScheduledPreview();
  stopRendering();
  if (!livePreview.checked) {
    statusLabel.textContent = 'Сцена изменена. Нажми «Перерисовать», чтобы обновить картинку.';
    return;
  }
  statusLabel.textContent = 'Обновляем превью…';
  previewTimer = setTimeout(() => render(true), 350);
}
const editor = mountSceneEditor(schedulePreview);
ready = true;
stopRendering();
renderButton.addEventListener('click', () => render(false));
livePreview.addEventListener('change', () => {
  clearScheduledPreview();
  if (livePreview.checked) schedulePreview();
});
function cancelRender() {
  clearScheduledPreview();
  stopRendering();
  statusLabel.textContent = 'Отменено. Последнее превью сохранено.';
}
cancelButton.addEventListener('click', cancelRender);
window.addEventListener('pagehide', () => {
  clearScheduledPreview();
  stopRendering();
  telegram.dispose();
});
window.addEventListener('pageshow', (event) => {
  if (event.persisted) location.reload();
});
schedulePreview();
