// Move the live canvas into a modal so progressive updates remain visible.
export function mountImageViewer(canvas, root = document) {
  const opener = root.querySelector('#open-preview');
  const dialog = root.querySelector('#image-viewer');
  const frame = root.querySelector('#viewer-frame');
  const close = root.querySelector('#close-preview');
  const render = root.querySelector('#render');
  const cancel = root.querySelector('#cancel');
  const status = root.querySelector('#status');
  const actions = root.querySelector('#viewer-actions');
  const footer = root.querySelector('#viewer-footer');
  const origins = [canvas, render, cancel, status].map((node) => {
    const marker = root.createComment('viewer return position');
    node.before(marker);
    return { node, marker };
  });
  let previousOverflow;
  opener.addEventListener('click', () => {
    if (dialog.open) return;
    previousOverflow = root.body.style.overflow;
    frame.append(canvas);
    actions.append(render, cancel);
    footer.append(status);
    dialog.showModal();
    root.body.style.overflow = 'hidden';
  });
  close.addEventListener('click', () => dialog.close());
  dialog.addEventListener('click', (event) => {
    if (event.target === frame || event.target === dialog) dialog.close();
  });
  dialog.addEventListener('close', () => {
    for (const { node, marker } of origins) marker.after(node);
    root.body.style.overflow = previousOverflow;
    opener.focus({ preventScroll: true });
  });
}
