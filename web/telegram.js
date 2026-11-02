// Telegram is an optional host. No user identity or launch data is sent anywhere.
export function connectTelegram({
  app = globalThis.Telegram?.WebApp,
  root = document.documentElement,
  onRender,
  onCancel,
} = {}) {
  const noop = {
    active: false,
    nativeButton: false,
    setState() {},
    feedback() {},
    dispose() {},
  };
  if (!app?.initData) return noop;
  // Host-specific failures must never prevent the ordinary page from working.
  const attempt = (fn) => {
    try {
      fn();
    } catch {
      /* unsupported client feature */
    }
  };
  const supported = (version) => app.isVersionAtLeast?.(version) === true;
  let ready = false;
  let busy = false;
  const theme = () => {
    root.style.colorScheme = app.colorScheme === 'light' ? 'light' : 'dark';
    if (supported('6.1')) {
      attempt(() => app.setHeaderColor('bg_color'));
      attempt(() => app.setBackgroundColor(app.themeParams?.bg_color || '#10151f'));
    }
  };
  const render = () => {
    if (!ready) return;
    if (busy) onCancel?.();
    else onRender?.();
  };
  attempt(() => app.ready());
  attempt(() => app.expand());
  theme();
  attempt(() => app.onEvent('themeChanged', theme));
  let nativeButton = false;
  try {
    app.MainButton.onClick(render);
    app.MainButton.setText('Готовим сцену…');
    app.MainButton.disable();
    app.MainButton.show();
    nativeButton = true;
  } catch {
    attempt(() => app.MainButton.offClick(render));
    attempt(() => app.MainButton.hide());
  }
  return {
    active: true,
    nativeButton,
    setState(next) {
      ready = next.ready;
      busy = next.busy;
      const canCancel = typeof onCancel === 'function';
      let buttonText = ready ? 'Перерисовать' : 'Готовим сцену…';
      if (busy) buttonText = canCancel ? 'Отменить' : 'Перерисовываем…';
      attempt(() => app.MainButton.setText(buttonText));
      attempt(() =>
        busy ? app.MainButton.showProgress(canCancel) : app.MainButton.hideProgress(),
      );
      attempt(() =>
        ready && (!busy || canCancel) ? app.MainButton.enable() : app.MainButton.disable(),
      );
      if (supported('6.2'))
        attempt(() => (busy ? app.enableClosingConfirmation() : app.disableClosingConfirmation()));
    },
    feedback(success) {
      if (supported('6.1'))
        attempt(() => app.HapticFeedback.notificationOccurred(success ? 'success' : 'error'));
    },
    dispose() {
      attempt(() => app.offEvent('themeChanged', theme));
      attempt(() => app.MainButton.offClick(render));
      attempt(() => app.MainButton.hide());
      if (supported('6.2')) attempt(() => app.disableClosingConfirmation());
    },
  };
}
