import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0253 triple on/like stutter', () => {
  it('collapses a run of 3+ on or like (any case) to the first copy', () => {
    for (const [raw, want] of [
      ['on on On the replay modal', 'On the replay modal.'],
      [
        'like on on On the replay modal please',
        'Like on the replay modal please.',
      ],
      ['turn it on on on Friday please', 'Turn it on Friday please.'],
      [
        'Can you add a toggle for each like on on on the modal',
        'Can you add a toggle for each like on the modal?',
      ],
      ['like like like to ping an API', 'Like to ping an API.'],
      [
        'easiest for you to like like uh like to ping',
        'Easiest for you to like to ping.',
      ],
      [
        'she was like LIKE like totally shocked today',
        'She was like totally shocked today.',
      ],
      [
        'Please make it like like like a smooth fade',
        'Please make it like a smooth fade.',
      ],
      [
        'Are are are we always fetching on hover',
        'Are we always fetching on hover?',
      ],
      ['the the the cat sat on the mat', 'The cat sat on the mat.'],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses doubles and near-misses', () => {
    for (const [raw, want] of [
      ['turn it on on Friday please', 'Turn it on on Friday please.'],
      [
        'focus on On boarding the new hires',
        'Focus on On boarding the new hires.',
      ],
      [
        'I was like like whatever about it',
        'I was like like whatever about it.',
      ],
      [
        'like Like it or not we still ship today',
        'Like Like it or not we still ship today.',
      ],
      [
        'like a like a smooth fade across the board',
        'Like a like a smooth fade across the board.',
      ],
      [
        'on, on the other hand we wait today',
        'On, on the other hand we wait today.',
      ],
      ['like, like I said before today', 'Like, like I said before today.'],
      ['why Why did we make a new Thing', 'Why Why did we make a new Thing.'],
      ['you are Are you sure about that', 'You are Are you sure about that.'],
      [
        'The results are are interesting today',
        'The results are are interesting today.',
      ],
      ['is is that correct though', 'Is is that correct though.'],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
