// This file is served as the optional SDK only in the simulated Telegram host.
const native = parent.document.querySelector('#native-render');
const buttonUnavailable = new URL(parent.location.href).searchParams.get('fallback') === 'true';
const styles = {
  'bg-color': '#000000',
  'text-color': '#ffffff',
  'hint-color': '#999999',
  'secondary-bg-color': '#1c1c1d',
  'button-color': '#3e86fa',
  'button-text-color': '#ffffff',
};
for (const [key, value] of Object.entries(styles))
  document.documentElement.style.setProperty('--tg-theme-' + key, value);
window.Telegram = {
  WebApp: {
    initData: 'layout-test-only',
    colorScheme: 'dark',
    themeParams: { bg_color: '#000000' },
    isVersionAtLeast: () => true,
    ready() {},
    expand() {},
    setHeaderColor() {},
    setBackgroundColor() {},
    onEvent() {},
    offEvent() {},
    enableClosingConfirmation() {},
    disableClosingConfirmation() {},
    HapticFeedback: { notificationOccurred() {} },
    MainButton: {
      onClick(fn) {
        native.onclick = fn;
      },
      offClick() {
        native.onclick = null;
      },
      setText(text) {
        native.textContent = text;
      },
      disable() {
        native.disabled = true;
      },
      enable() {
        native.disabled = false;
      },
      show() {
        if (buttonUnavailable) throw new Error('unsupported native button');
        native.style.display = 'block';
      },
      hide() {
        native.style.display = 'none';
      },
      showProgress(active) {
        if (!active) native.disabled = true;
      },
      hideProgress() {},
    },
  },
};
