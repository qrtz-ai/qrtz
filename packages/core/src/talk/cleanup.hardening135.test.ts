import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0251 triple are stutter', () => {
  it('collapses a run of 3+ are (any case) to the first copy', () => {
    for (const [raw, want] of [
      [
        'Are are are we always fetching on hover',
        'Are we always fetching on hover?',
      ],
      ['are are are we sure about this', 'Are we sure about this?'],
      ['we are are are going tomorrow', 'We are going tomorrow.'],
      ['Are are are are we done', 'Are we done'],
      ['Are ARE are we ready yet', 'Are we ready yet?'],
      ['they are Are ARE coming over later', 'They are coming over later.'],
      ['so are are are the results in', 'So are the results in.'],
      [
        'And then are are are we shipping tonight',
        'And then are we shipping tonight.',
      ],
      ['the the the cat sat on the mat', 'The cat sat on the mat.'],
      [
        "Yeah, it's still flying in, it's It's not correct",
        "Yeah, it's still flying in, it's not correct.",
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses doubles and near-misses', () => {
    for (const [raw, want] of [
      ['you are Are you sure about that', 'You are Are you sure about that.'],
      ['they are Are they coming today', 'They are Are they coming today.'],
      ['What are Are we doing next', 'What are Are we doing next.'],
      ['there are Are there any left', 'There are Are there any left.'],
      [
        'The results are are interesting today',
        'The results are are interesting today.',
      ],
      ['Are Are we ready to ship', 'Are Are we ready to ship.'],
      ['turn it on on Friday please', 'Turn it on on Friday please.'],
      ['is is that correct though', 'Is is that correct though.'],
      ['we are, are we sure', 'We are, are we sure.'],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
