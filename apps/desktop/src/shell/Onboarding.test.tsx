// @vitest-environment jsdom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { Onboarding, onboardingPermissions, setupState } from './Onboarding';
import { fakeHost } from './onboarding.fake';

// QRT-375: these suites exercise nano's rows as a dev build (?dev=1) with experimental apps on shows them.
history.replaceState(null, '', '/?dev=1');

(
  globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }
).IS_REACT_ACT_ENVIRONMENT = true;

const tick = () =>
  act(async () => {
    await new Promise((r) => setTimeout(r, 10));
  });

async function mount(host: ReturnType<typeof fakeHost>) {
  window.qrtzTransport = host.transport;
  const onDone = vi.fn();
  const el = document.createElement('div');
  document.body.appendChild(el);
  const root = createRoot(el);
  await act(async () => root.render(<Onboarding onDone={onDone} />));
  await tick();
  const q = (id: string) =>
    el.querySelector<HTMLElement>(`[data-testid="${id}"]`);
  const state = (id: string) => q(`onboarding-perm-${id}-state`)?.textContent;
  const click = async (id: string) => {
    await act(async () => q(id)!.click());
    await tick();
  };
  return { el, root, q, state, click, onDone };
}

afterEach(() => {
  delete (window as { qrtzTransport?: unknown }).qrtzTransport;
  document.body.innerHTML = '';
});

describe('setupState (ADR 0184)', () => {
  it('maps the live status to granted / not granted / needs restart / checking', () => {
    expect(setupState('granted', 'microphone', false)).toBe('granted');
    expect(setupState('denied', 'microphone', true)).toBe('not-granted');
    expect(setupState('not-determined', 'accessibility', false)).toBe(
      'not-granted',
    );
    // A screen recording grant reaches a running qrtz only after a relaunch.
    expect(setupState('denied', 'screen-recording', false)).toBe('not-granted');
    expect(setupState('denied', 'screen-recording', true)).toBe(
      'needs-restart',
    );
    expect(setupState('unsupported', 'microphone', false)).toBe('not-needed');
    expect(setupState(undefined, 'microphone', false)).toBe('checking');
    expect(setupState('unknown', 'microphone', false)).toBe('checking');
  });
});

describe('onboardingPermissions (ADR 0184 amendment)', () => {
  const ids = (cfg: unknown) => onboardingPermissions(cfg).map((d) => d.id);
  it('lists system audio only while nano is installed', () => {
    const nano = (installed: boolean) => ({
      app: { experimentalApps: true },
      modules: { nano: { installed } },
    });
    expect(ids(nano(true))).toEqual([
      'microphone',
      'screen-recording',
      'accessibility',
    ]);
    expect(ids(nano(false))).toEqual(['microphone', 'accessibility']);
    // Before config is read: only what every install needs (no flash of a nano-only row).
    expect(ids(null)).toEqual(['microphone', 'accessibility']);
  });
  it('QRT-375: a packaged build never lists nano rows, even installed with experimental apps on', () => {
    history.replaceState(null, '', '/');
    try {
      expect(
        ids({
          app: { experimentalApps: true },
          modules: { nano: { installed: true } },
        }),
      ).toEqual(['microphone', 'accessibility']);
    } finally {
      history.replaceState(null, '', '/?dev=1');
    }
  });
  it('never throws on a config without that app entry (modules?.[only]?.installed)', () => {
    for (const cfg of [{}, { modules: {} }, { modules: { nano: null } }])
      expect(() => ids(cfg)).not.toThrow();
  });
});

describe('permissions onboarding (ADR 0184)', () => {
  it('without nano installed: only microphone and accessibility, never asks for system audio', async () => {
    const host = fakeHost({ nanoInstalled: false });
    const { el, state, q, click, root } = await mount(host);
    expect(state('microphone')).toBe('not granted');
    expect(state('accessibility')).toBe('not granted');
    expect(q('onboarding-perm-screen-recording-state')).toBeNull();
    expect(el.textContent).not.toContain('system audio');
    expect(el.textContent).not.toContain('record calls');
    expect(el.textContent).toBe(el.textContent!.toLowerCase());
    // Everything listed granted is enough to say open qrtz (system audio isn't needed yet).
    host.state.perms.microphone = 'granted';
    host.state.perms.accessibility = 'granted';
    await act(async () => window.dispatchEvent(new Event('focus')));
    await tick();
    expect(q('onboarding-continue')!.textContent).toBe('open qrtz');
    await click('onboarding-continue');
    expect(
      host
        .seen('permissions:request')
        .concat(host.seen('permissions:open-settings'))
        .map((c) => JSON.stringify(c.payload))
        .join(),
    ).not.toContain('screen-recording');
    act(() => root.unmount());
  });

  it('nano installed while it is open (config:changed): the system audio row appears', async () => {
    const host = fakeHost({ nanoInstalled: false });
    const { q, state, root } = await mount(host);
    expect(q('onboarding-perm-screen-recording-state')).toBeNull();
    host.state.nanoInstalled = true;
    await act(async () =>
      host.emit('config:changed', {
        onboarding: { done: false },
        app: { experimentalApps: true },
        modules: { nano: { installed: true } },
      }),
    );
    await tick();
    expect(state('screen-recording')).toBe('not granted');
    act(() => root.unmount());
  });

  it('shows microphone, system audio and accessibility with their live state, all lowercase', async () => {
    const host = fakeHost({ perms: { microphone: 'granted' } });
    const { el, state, q, root } = await mount(host);
    expect(state('microphone')).toBe('granted');
    expect(state('screen-recording')).toBe('not granted');
    expect(state('accessibility')).toBe('not granted');
    expect(q('onboarding-perm-microphone-allow')).toBeNull();
    expect(q('onboarding-perm-accessibility-allow')).not.toBeNull();
    expect(el.textContent).toContain('system audio');
    expect(el.textContent).toBe(el.textContent!.toLowerCase());
    // The shared page layout.
    expect(el.querySelector('[data-page]')).not.toBeNull();
    // QRT-376: opening the onboarding only reads flow's model status (scoped to echo); nothing downloads.
    expect(
      host.state.calls
        .filter((c) => c.channel.startsWith('speech:'))
        .map((c) => [c.channel, c.payload]),
    ).toEqual([['speech:status', { extensions: ['echo'] }]]);
    act(() => root.unmount());
  });

  it('allow asks macOS for that permission and the row follows the answer', async () => {
    const host = fakeHost();
    const { state, click, root } = await mount(host);
    await click('onboarding-perm-microphone-allow');
    expect(host.seen('permissions:request')[0]!.payload).toEqual({
      permission: 'microphone',
    });
    expect(state('microphone')).toBe('granted');
    act(() => root.unmount());
  });

  it('settings opens the pane; system audio not on yet says needs restart and offers restart qrtz', async () => {
    const host = fakeHost();
    host.state.grantOnRequest = false;
    const { state, click, q, root } = await mount(host);
    await click('onboarding-perm-screen-recording-open');
    expect(host.seen('permissions:open-settings')[0]!.payload).toEqual({
      pane: 'screen-recording',
    });
    expect(state('screen-recording')).toBe('needs restart');
    await click('onboarding-perm-screen-recording-restart');
    expect(host.seen('app:restart')).toHaveLength(1);
    // Accessibility asked but still off is just not granted (no restart needed).
    await click('onboarding-perm-accessibility-allow');
    expect(state('accessibility')).toBe('not granted');
    expect(q('onboarding-perm-accessibility-restart')).toBeNull();
    act(() => root.unmount());
  });

  it('re-reads the status when the window regains focus, and on permissions:changed', async () => {
    const host = fakeHost();
    const { state, root } = await mount(host);
    const before = host.seen('permissions:get').length;
    host.state.perms.accessibility = 'granted';
    await act(async () => window.dispatchEvent(new Event('focus')));
    await tick();
    expect(host.seen('permissions:get').length).toBe(before + 1);
    expect(state('accessibility')).toBe('granted');
    await act(async () =>
      host.emit('permissions:changed', {
        ...host.state.perms,
        microphone: 'granted',
      }),
    );
    await tick();
    expect(state('microphone')).toBe('granted');
    act(() => root.unmount());
  });

  it.each(['onboarding-continue', 'onboarding-skip'])(
    '%s finishes through the host (onboarding:finish), never config:set',
    async (button) => {
      const host = fakeHost();
      const { click, onDone, root } = await mount(host);
      await click(button);
      expect(host.seen('onboarding:finish')).toHaveLength(1);
      expect(host.seen('config:set')).toHaveLength(0);
      expect(host.state.done).toBe(true);
      expect(onDone).toHaveBeenCalledOnce();
      act(() => root.unmount());
    },
  );

  it('keeps the continue button hidden until config:get answers', async () => {
    const host = fakeHost({
      perms: {
        microphone: 'granted',
        'screen-recording': 'granted',
        accessibility: 'granted',
      },
    });
    let release!: () => void;
    const gate = new Promise<void>((r) => (release = r));
    const invoke = host.transport.invoke;
    host.transport.invoke = async (e) => {
      if (e.channel === 'config:get') await gate;
      return invoke(e);
    };
    const { q, root } = await mount(host);
    expect(q('onboarding-continue')).toBeNull();
    expect(q('onboarding-skip')).not.toBeNull();
    await act(async () => release());
    await tick();
    expect(q('onboarding-continue')!.textContent).toBe('open qrtz');
    act(() => root.unmount());
  });

  it('says open qrtz when everything is granted', async () => {
    const host = fakeHost({
      perms: {
        microphone: 'granted',
        'screen-recording': 'granted',
        accessibility: 'granted',
      },
    });
    const { q, root } = await mount(host);
    expect(q('onboarding-continue')!.textContent).toBe('open qrtz');
    act(() => root.unmount());
  });
});

describe("flow's speech model in the onboarding (QRT-376)", () => {
  it('missing: the row offers the download, and "download and continue" starts it (scoped to echo) then finishes', async () => {
    const host = fakeHost({ speechInstalled: false });
    const { el, q, click, onDone, root } = await mount(host);
    expect(q('onboarding-speech-model-status')?.textContent).toContain(
      'not downloaded',
    );
    expect(el.textContent).toContain('hugging face');
    expect(q('onboarding-continue')!.textContent).toBe('download and continue');
    await click('onboarding-continue');
    expect(host.seen('speech:download').map((c) => c.payload)).toEqual([
      { extensions: ['echo'] },
    ]);
    expect(host.seen('onboarding:finish')).toHaveLength(1);
    expect(onDone).toHaveBeenCalled();
    act(() => root.unmount());
  });

  it('downloads in place with live progress, says installing after the last byte, pauses and resumes', async () => {
    const host = fakeHost({ speechInstalled: false });
    const { q, click, onDone, root } = await mount(host);
    await click('onboarding-speech-model-download');
    expect(host.seen('speech:download')).toHaveLength(1);
    const status = () => q('onboarding-speech-model-status')?.textContent;
    const push = async (download: object) => {
      host.state.speech.download =
        download as typeof host.state.speech.download;
      await act(async () =>
        host.emit('speech:status', structuredClone(host.state.speech)),
      );
      await tick();
    };
    await push({ state: 'downloading', done: 232_000_000, total: 464_000_000 });
    expect(status()).toContain('downloading · 50%');
    // Still downloading: continue finishes without starting it again.
    expect(q('onboarding-continue')!.textContent).not.toBe(
      'download and continue',
    );
    await push({ state: 'installing', done: 464_000_000, total: 464_000_000 });
    expect(status()).toBe('installing…');
    expect(q('onboarding-speech-model-cancel')).toBeNull();
    await push({ state: 'downloading', done: 300_000_000, total: 464_000_000 });
    await click('onboarding-speech-model-cancel');
    expect(host.seen('speech:pause').map((c) => c.payload)).toEqual([
      { extensions: ['echo'] },
    ]);
    await push({ state: 'paused', done: 300_000_000, total: 464_000_000 });
    await click('onboarding-speech-model-resume');
    expect(host.seen('speech:download')).toHaveLength(2);
    host.state.speech.installed = '1.0.0';
    await push({ state: 'idle', done: 0, total: 0 });
    expect(status()).toContain('on this mac');
    await click('onboarding-continue');
    expect(host.seen('speech:download')).toHaveLength(2);
    expect(onDone).toHaveBeenCalled();
    act(() => root.unmount());
  });

  it('a failed download says why and offers retry; skip never downloads', async () => {
    const host = fakeHost({ speechInstalled: false });
    host.state.speechFails = true;
    const { q, click, onDone, root } = await mount(host);
    await click('onboarding-speech-model-download');
    expect(q('onboarding-error')?.textContent).toContain(
      "couldn't reach the download server",
    );
    host.state.speech.download = {
      state: 'error',
      done: 0,
      total: 0,
      error: "couldn't reach the download server",
    };
    await act(async () =>
      host.emit('speech:status', structuredClone(host.state.speech)),
    );
    await tick();
    expect(q('onboarding-speech-model-retry')).not.toBeNull();
    host.state.speechFails = false;
    await click('onboarding-speech-model-retry');
    expect(host.seen('speech:download')).toHaveLength(2);
    expect(onDone).not.toHaveBeenCalled();
    act(() => root.unmount());

    const host2 = fakeHost({ speechInstalled: false });
    const m2 = await mount(host2);
    await m2.click('onboarding-skip');
    expect(host2.seen('speech:download')).toHaveLength(0);
    expect(m2.onDone).toHaveBeenCalled();
    act(() => m2.root.unmount());
  });
});
