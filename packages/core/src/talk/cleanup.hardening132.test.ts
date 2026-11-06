import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0248 one-word Cap contraction restart', () => {
  it('collapses mid-sentence Cap contraction restarts', () => {
    for (const [raw, want] of [
      [
        "Yeah, it's still flying in, it's It's not correct",
        "Yeah, it's still flying in, it's not correct.",
      ],
      [
        "I don't Don't want that change right now",
        "I don't want that change right now.",
      ],
      ["we can't Can't really ship today", "We can't really ship today."],
      [
        "I think that's That's the one we need",
        "I think that's the one we need.",
      ],
      [
        "maybe you're You're right about the fade",
        "Maybe you're right about the fade.",
      ],
      ["and there's There's no way around it", "And there's no way around it."],
      ["if it isn't Isn't ready we wait", "If it isn't ready we wait."],
      ["they won't Won't allow that path", "They won't allow that path."],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses non-contraction Cap and both-Cap forms', () => {
    for (const [raw, want] of [
      ["it's it's not a big deal for us", "It's it's not a big deal for us."],
      [
        "It's It's fine at the start of the line",
        "It's It's fine at the start of the line.",
      ],
      [
        'will Will smith play the role tonight',
        'Will Will smith play the role tonight.',
      ],
      [
        'like Like it or not we still ship',
        'Like Like it or not we still ship.',
      ],
      [
        'is Is Building the primary question here',
        'Is Is Building the primary question here.',
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
        'so So then we left the room quietly',
        'So So then we left the room quietly.',
      ],
      [
        'like a like a smooth fade-out works',
        'Like a like a smooth fade-out works.',
      ],
      ['Can you Can you host the prototype', 'Can you host the prototype?'],
      [
        'We moved to new york New York last year',
        'We moved to new york New York last year.',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
