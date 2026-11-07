import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0256 period-gap Cap-then-lower / both-lower restart', () => {
  it('collapses Cap-then-lower and both-lower restarts across a single period', () => {
    for (const [raw, want] of [
      [
        'And then the output should be. should be a simple lightweight React animation today',
        'And then the output should be a simple lightweight React animation today.',
      ],
      [
        'Thank you. thank you for the review today',
        'Thank you for the review today.',
      ],
      [
        'Can you. can you host the prototype today',
        'Can you host the prototype today?',
      ],
      [
        "What's. what's the meeting pass code today",
        "What's the meeting pass code today?",
      ],
      ['Is that. is that one good today', 'Is that one good today?'],
      ['You have to. you have to ship it today', 'You have to ship it today.'],
      [
        "How's it going. how's it going for you today",
        "How's it going for you today?",
      ],
      [
        "Let's just. let's just make the feature flag today",
        "Let's just make the feature flag today.",
      ],
      ['Is that is that one good today', 'Is that one good today?'],
      [
        'Is that... is that one good for us now',
        'Is that one good for us now?',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses Cap Cap across period, space-only both-lower MW, and discourse No.no', () => {
    for (const [raw, want] of [
      [
        'I will go. I will go to the store today',
        'I will go. I will go to the store today.',
      ],
      [
        'I talk clearly. I talk clearly about that today',
        'I talk clearly. I talk clearly about that today.',
      ],
      ['like a like a simple path today', 'Like a like a simple path today.'],
      [
        'the starting deck the starting deck looks fine today',
        'The starting deck the starting deck looks fine today.',
      ],
      ['is that is that one good today', 'Is that is that one good today.'],
      [
        'Where where we put the snap summon today',
        'Where where we put the snap summon today.',
      ],
      [
        'No. no just push it for us today please',
        'No no just push it for us today please.',
      ],
      [
        'you have to you have to ship it today',
        'You have to you have to ship it today.',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
