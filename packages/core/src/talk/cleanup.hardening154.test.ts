import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0261 lowercase "than than" stutter', () => {
  it('collapses', () => {
    const cases: Array<[string, string]> = [
      [
        'of a symbolic representation than than an actual yeah, if it is an array',
        'Of a symbolic representation than an actual yeah, if it is an array.',
      ],
      [
        "It's more of a symbolic representation than than an actual list today",
        "It's more of a symbolic representation than an actual list today.",
      ],
      [
        'This is way faster than than the old build today',
        'This is way faster than the old build today.',
      ],
      [
        'I would rather walk than than drive to the office today',
        'I would rather walk than drive to the office today.',
      ],
      [
        'It costs more than than than we planned today',
        'It costs more than we planned today.',
      ],
      ['It is more than\tthan before today', 'It is more than before today.'],
    ];
    for (const [raw, want] of cases) expect(ruleCleanup(raw)).toBe(want);
  });

  it('keeps Cap, comma, and non-repeat cases', () => {
    for (const raw of [
      'It is faster than Than Shwe expected today',
      'Than than is not a word today',
      'I said than THAN on purpose today',
      'It is bigger than, than the other one today',
      "It's more than thanks today",
      'It is more then than before today',
    ])
      expect(ruleCleanup(raw)).toBe(`${raw}.`);
  });
});
