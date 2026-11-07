import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0255 Cap-then-lower multi-word restart', () => {
  it('collapses Cap-then-lower multi-word restart phrases', () => {
    for (const [raw, want] of [
      ['Is that is that one good today', 'Is that one good today?'],
      ['You have to you have to ship it today', 'You have to ship it today.'],
      ['Can we can we try that again today', 'Can we try that again today?'],
      [
        'What are the what are the deciding factors today',
        'What are the deciding factors today?',
      ],
      [
        "How's it going how's it going for you today",
        "How's it going for you today?",
      ],
      [
        "Let's just let's just make the feature flag today",
        "Let's just make the feature flag today.",
      ],
      [
        'This is this is the right path for us today',
        'This is the right path for us today.',
      ],
      [
        'Is that... is that one good for us now',
        'Is that one good for us now?',
      ],
      [
        'We could just have we could just have our configuration today',
        'We could just have our configuration today.',
      ],
      [
        'I already did I sent a message. Is that is that one good?',
        'I already did I sent a message. Is that one good?',
      ],
      [
        'Can you Can you host the prototype today',
        'Can you host the prototype today?',
      ],
      [
        "How's it going How's it going for the team today",
        "How's it going for the team today?",
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses both-lower multi-word, unequal, and Cap Cap proper names', () => {
    for (const [raw, want] of [
      ['is that is that one good today', 'Is that is that one good today.'],
      [
        'you have to you have to ship it today',
        'You have to you have to ship it today.',
      ],
      [
        'what are the what are the options today',
        'What are the what are the options today.',
      ],
      [
        'can we can we try that again today',
        'Can we can we try that again today.',
      ],
      ['Is this is that one good today', 'Is this is that one good today.'],
      [
        'Where where we put the snap summon today',
        'Where where we put the snap summon today.',
      ],
      [
        'No, no just push it for us today please',
        'No, no just push it for us today please.',
      ],
      [
        'New York New York is the place today',
        'New York New York is the place today.',
      ],
      [
        'make make sure it uses CSS today please',
        'Make make sure it uses CSS today please.',
      ],
      [
        'the starting deck the starting deck looks fine today',
        'The starting deck the starting deck looks fine today.',
      ],
      ["What's what's the meeting pass code", "What's the meeting pass code?"],
      [
        'on on On the replay modal today please',
        'On the replay modal today please.',
      ],
      [
        'Are are are we always fetching on hover today',
        'Are we always fetching on hover today?',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
