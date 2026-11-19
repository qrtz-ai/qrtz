// @vitest-environment jsdom
import { describe, expect, it, vi } from 'vitest';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { createMemoryRouter, RouterProvider } from 'react-router';
import { ConfigSchema, defineSubApp } from '@qrtz/core';

// A dev build (?dev=1): nano and ctrl are reachable there (QRT-375); packaged builds are covered in modules.test.ts.
history.replaceState(null, '', '/?dev=1');

(
  globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }
).IS_REACT_ACT_ENVIRONMENT = true;

// ADR 0146: an app switched off on the qrtz page (each test picks which).
const off = vi.hoisted(() => ({ id: 'echo' }));
vi.mock('./useModuleSwitches', () => ({
  useModuleSwitches: () =>
    ConfigSchema.parse({ modules: { [off.id]: { enabled: false } } }),
}));

vi.mock('./subapps', async () => {
  const { createRegistry } = await import('@qrtz/core');
  const app = (id: string) =>
    defineSubApp({
      manifest: { id, name: id, icon: id },
      load: async () => ({
        default: () => <div data-testid={`view-${id}`} />,
      }),
    });
  return {
    registry: createRegistry([
      app('note'),
      app('talk'),
      app('marketplace'),
      app('settings'),
    ]),
  };
});

const { routes } = await import('./App');

const mount = async (entry: string) => {
  const router = createMemoryRouter(routes, { initialEntries: [entry] });
  const el = document.createElement('div');
  document.body.appendChild(el);
  const root = createRoot(el);
  await act(async () => root.render(<RouterProvider router={router} />));
  await act(async () => {
    await new Promise((r) => setTimeout(r, 10));
  });
  const q = (id: string) => el.querySelector(`[data-testid="${id}"]`);
  return { router, root, q };
};

describe('module switches (ADR 0146)', () => {
  // ADR 0237: every first-party module is built into qrtz.app, so a switched-off one keeps its tab and its route:
  // its view shows the off state (its switch stays in the marketplace and settings; only echo's view holds one).
  it('a switched-off bundled module keeps its tab and its route', async () => {
    off.id = 'echo';
    const { router, root, q } = await mount('/talk');
    expect(q('nav-talk')).not.toBeNull();
    expect(q('nav-note')).not.toBeNull();
    expect(q('nav-apps')).not.toBeNull();
    expect(router.state.location.pathname).toBe('/talk');
    expect(q('view-talk')).not.toBeNull();
    root.unmount();
  });

  // QRT-375: an experimental app (nano, ctrl) that is off, as it always is when experimental apps is off, has
  // no tab and no route: a deep link to it lands on the top sidebar app.
  it('a switched-off experimental app has no tab and no route', async () => {
    off.id = 'nano';
    const { router, root, q } = await mount('/note/meetings/m1');
    expect(q('nav-note')).toBeNull();
    expect(q('nav-talk')).not.toBeNull();
    expect(router.state.location.pathname).not.toMatch(/^\/note/);
    expect(q('view-note')).toBeNull();
    root.unmount();
  });
});
