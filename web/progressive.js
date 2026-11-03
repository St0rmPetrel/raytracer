// Render complete low-resolution previews before the final image. The sum of
// earlier pixel counts is less than one third of the final pass for powers of 2.
export function renderSizes(target) {
  if (!Number.isInteger(target) || target < 1 || target > 1024) {
    throw new Error('Resolution must be an integer between 1 and 1024');
  }
  const sizes = [];
  for (let size = 32; size < target; size *= 2) sizes.push(size);
  sizes.push(target);
  return sizes;
}

export async function renderProgressively({
  scene,
  size,
  render,
  publish,
  yieldControl = () => new Promise((resolve) => setTimeout(resolve, 0)),
  now = () => performance.now(),
}) {
  const sizes = renderSizes(size);
  const started = now();
  for (const [index, current] of sizes.entries()) {
    const rgba = render(scene, current, current);
    publish({
      rgba,
      size: current,
      targetSize: size,
      elapsed: now() - started,
      pass: index + 1,
      passes: sizes.length,
      done: current === size,
    });
    // Allow frame delivery between passes; the UI continues painting separately.
    if (current !== size) await yieldControl();
  }
}

// Deliver bounded strips within every pass, including the expensive final one.
export async function renderTiledProgressively({
  scene,
  size,
  createRenderer,
  publish,
  yieldControl = () => new Promise((resolve) => setTimeout(resolve, 0)),
  now = () => performance.now(),
}) {
  const sizes = renderSizes(size);
  const started = now();
  for (const [index, current] of sizes.entries()) {
    const renderer = createRenderer(scene, current, current);
    try {
      const stripHeight = Math.max(1, Math.min(32, Math.floor(8192 / current)));
      for (let row = 0; row < current; row += stripHeight) {
        const rows = Math.min(stripHeight, current - row);
        const rgba = renderer.render_rows(row, rows);
        const passDone = row + rows === current;
        const done = passDone && current === size;
        publish({
          rgba,
          row,
          rows,
          size: current,
          targetSize: size,
          elapsed: now() - started,
          pass: index + 1,
          passes: sizes.length,
          passDone,
          done,
        });
        if (!done) await yieldControl();
      }
    } finally {
      renderer.free();
    }
  }
}
