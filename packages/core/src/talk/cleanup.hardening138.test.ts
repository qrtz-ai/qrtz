import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0254 Cap-then-lower contraction restart', () => {
  it('collapses Cap-then-lower one-word contraction doubles', () => {
    for (const [raw, want] of [
      ["What's what's the meeting pass code", "What's the meeting pass code?"],
      [
        "It's it's the top two things aren't fading now",
        "It's the top two things aren't fading now.",
      ],
      [
        "That's that's not true about the release today",
        "That's not true about the release today.",
      ],
      [
        "There's there's no way around this today",
        "There's no way around this today.",
      ],
      [
        "You're you're hearing a live stream of thoughts today",
        "You're hearing a live stream of thoughts today.",
      ],
      [
        "Can't can't really ship without the tests today",
        "Can't really ship without the tests today.",
      ],
      [
        "Won't won't allow that path for us today",
        "Won't allow that path for us today.",
      ],
      [
        "What's... what's the meeting pass code now",
        "What's the meeting pass code now?",
      ],
      [
        "Yeah, it's still flying in, it's It's not correct",
        "Yeah, it's still flying in, it's not correct.",
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses both-Cap, both-lower, and non-contraction near-misses', () => {
    for (const [raw, want] of [
      ["it's it's not a big deal for us", "It's it's not a big deal for us."],
      [
        "It's It's fine at the start of the line",
        "It's It's fine at the start of the line.",
      ],
      [
        "don't don't go down that path yet",
        "Don't don't go down that path yet.",
      ],
      [
        "O'Brien O'Brien wrote the release notes",
        "O'Brien O'Brien wrote the release notes.",
      ],
      [
        "Can't Can't really wait for us today",
        "Can't Can't really wait for us today.",
      ],
      [
        "What's What's the name of that cafe today",
        "What's What's the name of that cafe today.",
      ],
      [
        "that's that's not a problem for me today",
        "That's that's not a problem for me today.",
      ],
      [
        'Where where we put the snap summon today',
        'Where where we put the snap summon today.',
      ],
      [
        'make make sure it uses CSS today please',
        'Make make sure it uses CSS today please.',
      ],
      [
        "Here's what's happening with the launch today",
        "Here's what's happening with the launch today.",
      ],
      ['why Why did we make a new Thing', 'Why Why did we make a new Thing.'],
      [
        'like Like it or not we still ship today',
        'Like Like it or not we still ship today.',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
