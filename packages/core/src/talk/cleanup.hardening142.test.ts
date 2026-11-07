import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0258 Cap Cap verbal contraction across a period', () => {
  it('collapses', () => {
    const cases: Array<[string, string]> = [
      [
        "Like, I'm. I'm a senior engineer now today",
        "Like, I'm a senior engineer now today.",
      ],
      ["It's. It's the top two things today", "It's the top two things today."],
      ["Don't. Don't push that change today", "Don't push that change today."],
      [
        "What's. What's the meeting pass code today",
        "What's the meeting pass code today?",
      ],
      [
        "Can't. Can't we just ship the flag today",
        "Can't we just ship the flag today.",
      ],
      [
        "We're. We're late to the standup today",
        "We're late to the standup today.",
      ],
      [
        "That's. That's not true about the release today",
        "That's not true about the release today.",
      ],
      [
        "How's. How's it going for the team today",
        "How's it going for the team today?",
      ],
      [
        'And then the output should be. should be a simple path today',
        'And then the output should be a simple path today.',
      ],
      [
        "What's what's the meeting pass code today",
        "What's the meeting pass code today?",
      ],
    ];
    for (const [raw, want] of cases) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refusals stay', () => {
    const cases: Array<[string, string]> = [
      [
        'I will go. I will go tomorrow with the team',
        'I will go. I will go tomorrow with the team.',
      ],
      [
        'So. So we should ship the flag today',
        'So. So we should ship the flag today.',
      ],
      [
        'Yes. Yes I can do that with the team today',
        'Yes. Yes I can do that with the team today.',
      ],
      [
        'I. I think that works for the team today',
        'I. I think that works for the team today.',
      ],
      [
        'Where. Where is the release for the team today',
        'Where. Where is the release for the team today?',
      ],
      [
        "I'm I'm a senior engineer on the team today",
        "I'm I'm a senior engineer on the team today.",
      ],
      [
        "I'm... I'm a senior engineer on the team today",
        "I'm... I'm a senior engineer on the team today.",
      ],
      [
        "It's It's ready for review with the team today",
        "It's It's ready for review with the team today.",
      ],
      [
        "Talk to O'Brien. O'Brien said yes about the release today",
        "Talk to O'Brien. O'Brien said yes about the release today.",
      ],
      [
        "Bob's. Bob's car is ready for the drive today",
        "Bob's. Bob's car is ready for the drive today.",
      ],
    ];
    for (const [raw, want] of cases) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
