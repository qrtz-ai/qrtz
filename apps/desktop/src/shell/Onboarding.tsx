import { useCallback, useEffect, useMemo, useState } from 'react';
import {
  Button,
  Callout,
  OnboardingFrame,
  PermissionSetupList,
  PermissionSetupRow,
  SettingsSection,
  SpeechModelRow,
  Stack,
  Text,
  type IconName,
  type PermissionSetupState,
} from '@qrtz/ui';
import {
  isHiddenExperiment,
  loadConfig,
  type CalendarAccess,
  type ModuleId,
  type OsPermission,
  type PermissionsStatus,
  type SettingsPane,
} from '@qrtz/core';
import {
  QrtzClientProvider,
  useQrtz,
  useQrtzEvent,
  useSpeechSetup,
} from '@qrtz/core/react';
import { shellClient } from './shellClient';

/**
 * ADR 0184: the permissions qrtz can read and ask for up front, in the order the onboarding lists them. A row with
 * `only` is listed only while that app is installed (ADR 0184 amendment: system audio only with nano installed;
 * nano installed later asks for it the first time it records, or here when the onboarding is reopened).
 */
export const ONBOARDING_PERMISSIONS: readonly {
  id: OsPermission;
  pane: SettingsPane;
  icon: IconName;
  label: string;
  reason: string;
  only?: ModuleId;
}[] = [
  {
    id: 'microphone',
    pane: 'microphone',
    icon: 'mic',
    label: 'microphone',
    reason: 'flow hears you while you hold the key',
  },
  {
    id: 'screen-recording',
    pane: 'screen-recording',
    icon: 'screen-recording',
    label: 'system audio',
    reason: 'nano records the other side of a call',
    only: 'nano',
  },
  {
    id: 'accessibility',
    pane: 'accessibility',
    icon: 'accessibility',
    label: 'accessibility',
    reason: 'flow types where your cursor is; snap moves your windows',
  },
];

/** A grant a running qrtz only sees after a relaunch (macOS reads screen recording once per process). */
const RESTART_AFTER_GRANT: readonly OsPermission[] = ['screen-recording'];
/** How long qrtz keeps re-reading after an ask (then the window-focus re-read takes over). */
const WATCH_MS = 120_000;

/**
 * The rows to list for this config (the host's `modules.<id>.installed`, ADR 0169). Before config is read
 * (`null`), only the rows every install needs: a nano-only row never flashes in and out.
 */
export function onboardingPermissions(cfg: unknown) {
  const c = cfg == null ? null : loadConfig(cfg);
  // QRT-375: a hidden experimental app (nano) lists nothing.
  return ONBOARDING_PERMISSIONS.filter(
    (d) =>
      !d.only ||
      (!!c &&
        !isHiddenExperiment(c, d.only) &&
        (c.modules?.[d.only]?.installed ?? false)),
  );
}

/**
 * ADR 0217: nano's optional Apple Calendar row, listed only while nano is installed. It is not an OS permission
 * qrtz needs (nothing waits on it, so it never counts toward "open qrtz"), and its allow is nano's connect.
 */
export function showCalendarRow(cfg: unknown) {
  if (cfg == null) return false;
  const c = loadConfig(cfg);
  return (
    !isHiddenExperiment(c, 'nano') && (c.modules?.nano?.installed ?? false)
  );
}

/** The calendar row's state from the calendar access state (ADR 0217 H1: the shell sees nothing more). */
export function calendarSetupState(
  st: CalendarAccess | null,
): PermissionSetupState {
  if (!st) return 'checking';
  if (st.access === 'granted') return 'granted';
  if (st.access === 'unsupported') return 'not-needed';
  return 'not-granted';
}

/** One row's state from the live status and whether the user has asked for it yet. */
export function setupState(
  st: string | undefined,
  id: OsPermission,
  asked: boolean,
): PermissionSetupState {
  if (st === 'granted') return 'granted';
  if (st === 'unsupported') return 'not-needed';
  if (st === 'denied' || st === 'not-determined')
    return asked && RESTART_AFTER_GRANT.includes(id)
      ? 'needs-restart'
      : 'not-granted';
  return 'checking';
}

/** QRT-376: what the onboarding says about flow's speech model download (the consent). */
export const SPEECH_NOTE =
  "flow's speech model (parakeet) downloads once from hugging face, is checked against the fingerprint built into qrtz, and stays on this mac.";

/** What the primary button says: download first while the model is missing (it is the consent), else open/continue. */
export function primaryLabel(
  speech: 'missing' | 'busy' | 'ready' | 'none',
  allGranted: boolean,
): string {
  if (speech === 'missing') return 'download and continue';
  return allGranted ? 'open qrtz' : 'continue';
}

/**
 * The first-run onboarding (ADR 0184; shell-owned, sub apps unmounted): flow's speech model (QRT-376: downloaded
 * here, in the background, with its progress, pause and retry, scoped by the host to echo's models), then the macOS
 * permissions. "download and continue" starts the download and finishes; skip finishes without it. The download
 * keeps going after the onboarding closes and flow's page shows it. A permission left off is asked the first time
 * an app needs it.
 */
export function Onboarding({ onDone }: { onDone: () => void }) {
  const client = useMemo(shellClient, []);
  return (
    <QrtzClientProvider client={client}>
      <Steps onDone={onDone} />
    </QrtzClientProvider>
  );
}

function Steps({ onDone }: { onDone: () => void }) {
  const qrtz = useQrtz();
  const [perms, setPerms] = useState<PermissionsStatus | null>(null);
  const [asked, setAsked] = useState<{ p: OsPermission[]; at: number }>({
    p: [],
    at: 0,
  });
  const [error, setError] = useState<string | null>(null);
  const [finishing, setFinishing] = useState(false);
  const live = useQrtzEvent('permissions:changed');
  // Which apps are installed decides the rows (system audio only with nano), live as the host syncs qrtz.lock.
  const [cfg, setCfg] = useState<Record<string, unknown> | null>(null);
  const liveCfg = useQrtzEvent('config:changed');
  const fail = (e: Error) => setError(e.message);
  const [cal, setCal] = useState<CalendarAccess | null>(null);
  const calShown = showCalendarRow(cfg);
  // QRT-376: flow's speech model, scoped to echo's models (the host refuses anything else from the shell).
  const speechSetup = useSpeechSetup({ extensions: ['echo'] });
  const speech = speechSetup.speech;
  const speechState: 'missing' | 'busy' | 'ready' | 'none' = !speech
    ? 'none'
    : !speech.configured
      ? 'none'
      : ['checking', 'downloading', 'paused', 'installing'].includes(
            speech.download.state,
          )
        ? 'busy'
        : speech.installed
          ? 'ready'
          : 'missing';
  const downloadSpeech = () => {
    setError(null);
    return qrtz
      .invoke('speech:download', { extensions: ['echo'] })
      .then(() => undefined, fail);
  };
  const pauseSpeech = () => {
    setError(null);
    void qrtz
      .invoke('speech:pause', { extensions: ['echo'] })
      .then(() => undefined, fail);
  };

  const read = useCallback(
    () => void qrtz.invoke('permissions:get').then(setPerms, fail),
    [qrtz],
  );
  useEffect(() => {
    read();
    // Granted in System Settings while qrtz was in the background: re-read when the window regains focus.
    window.addEventListener('focus', read);
    return () => window.removeEventListener('focus', read);
  }, [read]);
  useEffect(() => {
    if (live) setPerms(live);
  }, [live]);
  useEffect(() => {
    void qrtz.invoke('config:get').then(setCfg, fail);
  }, [qrtz]);
  useEffect(() => {
    if (liveCfg) setCfg(liveCfg);
  }, [liveCfg]);
  // Reads only (calendar:access never prompts); the ask is the row's allow. ADR 0217 H1: the shell holds no
  // calendar.read, so it reads the access state alone, again when the window regains focus (System Settings).
  const readCal = useCallback(() => {
    if (calShown)
      void qrtz.invoke('calendar:access').then(setCal, () => setCal(null));
  }, [qrtz, calShown]);
  useEffect(() => {
    readCal();
    window.addEventListener('focus', readCal);
    return () => window.removeEventListener('focus', readCal);
  }, [readCal]);
  const connectCalendar = () => {
    setError(null);
    void qrtz.invoke('calendar:connect').then(setCal, fail);
  };
  const calState = calendarSetupState(cal);
  const shown = onboardingPermissions(cfg);
  // After an ask, re-read every second until the grant shows (System Settings may still be in front).
  const watching = asked.p.some(
    (p) => shown.some((d) => d.id === p) && perms?.[p] !== 'granted',
  );
  useEffect(() => {
    if (!watching) return;
    const id = setInterval(() => {
      if (Date.now() - asked.at > WATCH_MS) clearInterval(id);
      else read();
    }, 1000);
    return () => clearInterval(id);
  }, [watching, asked.at, read]);

  const markAsked = (p: OsPermission) =>
    setAsked((a) => ({ p: [...new Set([...a.p, p])], at: Date.now() }));
  const allow = (p: OsPermission) => {
    setError(null);
    markAsked(p);
    void qrtz
      .invoke('permissions:request', { permission: p })
      .then(setPerms, fail);
  };
  const open = (p: OsPermission, pane: SettingsPane) => {
    setError(null);
    markAsked(p);
    void qrtz
      .invoke('permissions:open-settings', { pane })
      .then(() => read(), fail);
  };
  const restart = () =>
    void qrtz.invoke('app:restart').then(() => undefined, fail);
  const finish = (withSpeech = false) => {
    setError(null);
    setFinishing(true);
    // "download and continue": the download starts first (it runs on in the host after this window moves on).
    const start =
      withSpeech && speechState === 'missing'
        ? downloadSpeech()
        : Promise.resolve();
    void start
      .then(() => qrtz.invoke('onboarding:finish'))
      .then(onDone, (e: Error) => {
        setFinishing(false);
        fail(e);
      });
  };

  const rows = shown.map((d) => ({
    ...d,
    state: setupState(perms?.[d.id], d.id, asked.p.includes(d.id)),
  }));
  const allGranted = rows.every(
    (r) => r.state === 'granted' || r.state === 'not-needed',
  );

  return (
    <OnboardingFrame
      title="set up qrtz"
      data-testid="onboarding"
      actions={
        <Button
          variant="ghost"
          size="sm"
          onClick={() => finish(false)}
          disabled={finishing}
          data-testid="onboarding-skip"
        >
          skip
        </Button>
      }
    >
      <Text muted data-testid="onboarding-intro">
        {shown.some((d) => d.id === 'screen-recording')
          ? 'qrtz needs its speech model and a few macos permissions to hear you, record calls and type for you.'
          : 'qrtz needs its speech model and a few macos permissions to hear you and type for you.'}{' '}
        the model downloads in the background while you allow them. skip, and
        each one is asked again the first time an app needs it.
      </Text>
      {error ? (
        <Callout tone="danger" title={error} data-testid="onboarding-error" />
      ) : null}
      {speechState === 'none' && speech ? null : (
        <SettingsSection title="speech" data-testid="onboarding-speech">
          <SpeechModelRow
            status={speech}
            onDownload={() => void downloadSpeech()}
            onCancel={pauseSpeech}
            data-testid="onboarding-speech-model"
          />
          <Text variant="caption" muted data-testid="onboarding-speech-note">
            {SPEECH_NOTE}
          </Text>
        </SettingsSection>
      )}
      <PermissionSetupList aria-label="permissions">
        {rows.map((r) => (
          <PermissionSetupRow
            key={r.id}
            id={r.id}
            icon={r.icon}
            label={r.label}
            reason={r.reason}
            state={r.state}
            {...(r.state === 'not-granted'
              ? { onAllow: () => allow(r.id) }
              : {})}
            {...(r.state === 'not-granted' || r.state === 'needs-restart'
              ? { onOpenSettings: () => open(r.id, r.pane) }
              : {})}
            {...(r.state === 'needs-restart' ? { onRestart: restart } : {})}
          />
        ))}
        {calShown ? (
          <PermissionSetupRow
            id="calendar"
            icon="calendar"
            label="calendar (optional)"
            reason={
              cal?.access === 'denied'
                ? 'turned off: allow qrtz in system settings › privacy › calendars'
                : "nano names notes after your meetings and lists what's next; read only, never leaves your mac"
            }
            state={calState}
            {...(calState === 'not-granted' && cal?.access !== 'denied'
              ? { onAllow: connectCalendar }
              : {})}
          />
        ) : null}
      </PermissionSetupList>
      <Stack dir="row" justify="end" gap={2}>
        {/* Hidden until config is read: the rows (and so "open qrtz" vs "continue") depend on it. */}
        {cfg == null ? null : (
          <Button
            variant="primary"
            onClick={() => finish(true)}
            disabled={finishing}
            data-testid="onboarding-continue"
          >
            {primaryLabel(speechState, allGranted)}
          </Button>
        )}
      </Stack>
    </OnboardingFrame>
  );
}
