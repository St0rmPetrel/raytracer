import init, { BrowserRenderer } from './pkg/raytracer.js';
import { renderTiledProgressively } from './progressive.js';
// Lazy initialization lets us report loading failures through the worker protocol.
let initialized;
self.onmessage = async ({ data }) => {
  try {
    initialized ??= init();
    await initialized;
    await renderTiledProgressively({
      scene: data.scene,
      size: data.size,
      createRenderer: (scene, w, h) => new BrowserRenderer(scene, w, h, data.samples ?? 4),
      publish: (frame) => self.postMessage(frame, [frame.rgba.buffer]),
    });
  } catch (error) {
    self.postMessage({ error: String(error) });
  }
};
