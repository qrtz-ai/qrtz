// ADR 0243 (echo cycle 126): unmarked confirmation tags (", right" / ", correct") end with "?".
import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0243 confirmation tags end with question mark', () => {
  it.each([
    ['That works for me, right', 'That works for me, right?'],
    ['The CSV is what you update, right', 'The CSV is what you update, right?'],
    [
      "You're editing the document I sent you, right",
      "You're editing the document I sent you, right?",
    ],
    [
      'There has to be a better way to load this, correct',
      'There has to be a better way to load this, correct?',
    ],
    ['You mean this one, correct', 'You mean this one, correct?'],
    [
      'We should have CI on the release pipeline, right',
      'We should have CI on the release pipeline, right?',
    ],
    [
      'I would surely know about it, right',
      'I would surely know about it, right?',
    ],
    ['Make sure the tests pass, correct', 'Make sure the tests pass, correct?'],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });
});

describe('ADR 0243 confirmation tag refusals stay', () => {
  it.each([
    ['Please turn left, right', 'Please turn left, right.'],
    ['Go up, down, left, right', 'Go up, down, left, right.'],
    ['Look north, right', 'Look north, right'],
    ['Copy that, right away', 'Copy that, right away.'],
    ['I got it right', 'I got it right.'],
    [
      'That answer is politically correct',
      'That answer is politically correct.',
    ],
    ["It's fine, right?", "It's fine, right?"],
    ['Am I right', 'Am I right'],
    ['Keep right at the fork', 'Keep right at the fork.'],
    ['Yes, right', 'Yes, right'],
    ['Then left, right', 'Then left, right'],
    ['Put it on the right', 'Put it on the right.'],
    [
      'Mark the first answer wrong, the second correct',
      'Mark the first answer wrong, the second correct.',
    ],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });
});
