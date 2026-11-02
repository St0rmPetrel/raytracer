import test from 'node:test';
import assert from 'node:assert/strict';
import { connectTelegram } from '../web/telegram.js';

function host(version = true) {
  const calls = [];
  const events = new Map();
  let click;
  const app = {
    initData: 'only-used-to-detect-host',
    colorScheme: 'light',
    themeParams: { bg_color: '#ffffff' },
    isVersionAtLeast: () => version,
    ready: () => calls.push('ready'),
    expand: () => calls.push('expand'),
    setHeaderColor: () => {},
    setBackgroundColor: () => {},
    onEvent: (name, cb) => events.set(name, cb),
    offEvent: (name) => events.delete(name),
    enableClosingConfirmation: () => calls.push('confirm'),
    disableClosingConfirmation: () => calls.push('no-confirm'),
    HapticFeedback: { notificationOccurred: (kind) => calls.push(kind) },
    MainButton: {
      onClick: (cb) => {
        click = cb;
      },
      offClick: () => {
        click = undefined;
      },
      setText: (text) => calls.push(text),
      enable: () => calls.push('enabled'),
      disable: () => calls.push('disabled'),
      show: () => calls.push('shown'),
      hide: () => calls.push('hidden'),
      showProgress: () => calls.push('progress'),
      hideProgress: () => calls.push('no-progress'),
    },
  };
  return { app, calls, events, click: () => click?.() };
}

test('ordinary browser needs no Telegram SDK', () => {
  const bridge = connectTelegram({ app: {}, root: { style: {} } });
  assert.equal(bridge.active, false);
  assert.equal(bridge.nativeButton, false);
  bridge.setState({ ready: true, busy: false });
  bridge.feedback(true);
  bridge.dispose();
});

test('native render button follows loading, ready, rendering and cancellation', () => {
  const h = host();
  let renders = 0;
  const bridge = connectTelegram({
    app: h.app,
    root: { style: {} },
    onRender: () => renders++,
  });
  assert.equal(bridge.nativeButton, true);
  h.click();
  assert.equal(renders, 0);
  bridge.setState({ ready: true, busy: false });
  h.click();
  assert.equal(renders, 1);
  bridge.setState({ ready: true, busy: true });
  h.click();
  assert.equal(renders, 1);
  assert.ok(h.calls.includes('progress'));
  assert.ok(h.calls.includes('confirm'));
  bridge.setState({ ready: true, busy: false });
  h.click();
  assert.equal(renders, 2);
  bridge.feedback(true);
  assert.ok(h.calls.includes('success'));
  bridge.dispose();
  h.click();
  assert.equal(renders, 2);
  assert.equal(h.events.size, 0);
});

test('one native button switches between rerender and active cancellation', () => {
  const h = host();
  let renders = 0;
  let cancellations = 0;
  let progressRemainsActive = false;
  h.app.MainButton.showProgress = (leaveActive) => {
    progressRemainsActive = leaveActive;
  };
  const bridge = connectTelegram({
    app: h.app,
    root: { style: {} },
    onRender: () => renders++,
    onCancel: () => cancellations++,
  });
  bridge.setState({ ready: true, busy: false });
  assert.ok(h.calls.includes('Перерисовать'));
  h.click();
  assert.equal(renders, 1);
  bridge.setState({ ready: true, busy: true });
  assert.ok(h.calls.includes('Отменить'));
  assert.equal(progressRemainsActive, true);
  h.click();
  assert.equal(cancellations, 1);
  assert.equal(renders, 1);
  bridge.setState({ ready: true, busy: false });
  h.click();
  assert.equal(renders, 2);
});

test('a failed native button setup leaves the page-button fallback available', () => {
  const h = host();
  h.app.MainButton.show = () => {
    throw new Error('unavailable');
  };
  const bridge = connectTelegram({ app: h.app, root: { style: {} } });
  assert.equal(bridge.active, true);
  assert.equal(bridge.nativeButton, false);
  assert.ok(h.calls.includes('hidden'));
});

test('theme follows Telegram updates', () => {
  const h = host();
  const root = { style: {} };
  connectTelegram({ app: h.app, root });
  assert.equal(root.style.colorScheme, 'light');
  h.app.colorScheme = 'dark';
  h.events.get('themeChanged')();
  assert.equal(root.style.colorScheme, 'dark');
});

test('old or partially unavailable host features do not break rendering', () => {
  const h = host(false);
  h.app.MainButton.showProgress = () => {
    throw new Error('unsupported');
  };
  const bridge = connectTelegram({ app: h.app, root: { style: {} } });
  bridge.setState({ ready: true, busy: true });
  bridge.setState({ ready: true, busy: false });
  bridge.feedback(false);
  assert.ok(!h.calls.includes('confirm'));
  assert.ok(!h.calls.includes('error'));
});
