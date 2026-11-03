import test from 'node:test';
import assert from 'node:assert/strict';
import { renderSizes, renderProgressively, renderTiledProgressively } from '../web/progressive.js';

test('preview passes increase to the exact final resolution without duplicates', () => {
  assert.deepEqual(renderSizes(1024), [32, 64, 128, 256, 512, 1024]);
  assert.deepEqual(renderSizes(512), [32, 64, 128, 256, 512]);
  assert.deepEqual(renderSizes(128), [32, 64, 128]);
  assert.deepEqual(renderSizes(32), [32]);
  assert.deepEqual(renderSizes(1), [1]);
  assert.deepEqual(renderSizes(100), [32, 64, 100]);
  for (const value of [0, 1025, NaN, Infinity, 1.5]) assert.throws(() => renderSizes(value));
});

test('publishes intermediate frames before continuing; completion only on final pass', async () => {
  const events = [];
  const frames = [];
  await renderProgressively({
    scene: 'scene',
    size: 128,
    render: (_, w, h) => {
      events.push(`render:${w}`);
      return new Uint8Array(w * h * 4);
    },
    publish: (frame) => {
      events.push(`publish:${frame.size}`);
      frames.push(frame);
    },
    yieldControl: async () => {
      events.push('yield');
    },
    now: () => 10,
  });
  assert.deepEqual(events, [
    'render:32',
    'publish:32',
    'yield',
    'render:64',
    'publish:64',
    'yield',
    'render:128',
    'publish:128',
  ]);
  assert.deepEqual(
    frames.map((f) => f.done),
    [false, false, true],
  );
  assert.deepEqual(
    frames.map((f) => f.pass),
    [1, 2, 3],
  );
  assert.ok(frames.every((f) => f.passes === 3 && f.targetSize === 128));
});

test('a failed pass does not publish a false completion', async () => {
  const frames = [];
  await assert.rejects(
    renderProgressively({
      scene: 'scene',
      size: 128,
      render: (_, w) => {
        if (w === 64) throw new Error('failed');
        return new Uint8Array(w * w * 4);
      },
      publish: (f) => frames.push(f),
      yieldControl: async () => {},
    }),
    /failed/,
  );
  assert.equal(frames.length, 1);
  assert.equal(frames[0].done, false);
});

test('final high resolution pass publishes bounded strips and frees each renderer', async () => {
  const frames = [];
  const freed = [];
  let yields = 0;
  await renderTiledProgressively({
    scene: 'scene',
    size: 1024,
    createRenderer: (_, width) => ({
      render_rows: (row, rows) => new Uint8Array(width * rows * 4),
      free: () => freed.push(width),
    }),
    publish: (frame) => frames.push(frame),
    yieldControl: async () => {
      yields++;
    },
    now: () => 10,
  });
  const final = frames.filter((frame) => frame.size === 1024);
  assert.equal(final.length, 128);
  assert.deepEqual(
    final.map((frame) => frame.row),
    Array.from({ length: 128 }, (_, i) => i * 8),
  );
  assert.ok(
    frames.every(
      (frame) =>
        frame.rgba.length === frame.size * frame.rows * 4 && frame.size * frame.rows <= 8192,
    ),
  );
  assert.equal(frames.filter((frame) => frame.done).length, 1);
  assert.equal(frames.at(-1).done, true);
  assert.deepEqual(freed, [32, 64, 128, 256, 512, 1024]);
  assert.equal(yields, frames.length - 1);
});

test('a failed strip frees WASM memory without reporting completion', async () => {
  let freed = false;
  const frames = [];
  await assert.rejects(
    renderTiledProgressively({
      scene: 'scene',
      size: 64,
      createRenderer: (_, size) => ({
        render_rows: () => {
          throw new Error('failed strip');
        },
        free: () => {
          freed = true;
        },
      }),
      publish: (frame) => frames.push(frame),
    }),
    /failed strip/,
  );
  assert.equal(freed, true);
  assert.deepEqual(frames, []);
});
