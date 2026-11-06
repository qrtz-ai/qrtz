// ADR 0242 (echo cycle 125): spoken list "comma"/"period" become ","/".".
import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

const priya = [{ term: 'Priya', aliases: ['pria'] }];

describe('ADR 0242 spoken list comma and period become marks', () => {
  it.each([
    [
      'Bring the charger comma the cable comma and the adapter period',
      'Bring the charger, the cable, and the adapter.',
    ],
    [
      'We need eggs comma milk comma and bread period',
      'We need eggs, milk, and bread.',
    ],
    [
      'Bring the charger, the cable, and the adapter period.',
      'Bring the charger, the cable, and the adapter.',
    ],
    [
      'Grab pens comma paper comma and tape period',
      'Grab pens, paper, and tape.',
    ],
    [
      'Pack socks comma shirts comma and a jacket period',
      'Pack socks, shirts, and a jacket.',
    ],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });
});

describe('ADR 0242 spoken comma and period as nouns stay', () => {
  it.each([
    ['Add a comma after the name.', 'Add a comma after the name.'],
    ['The trial period ends Friday.', 'The trial period ends Friday.'],
    ['This is a period of growth.', 'This is a period of growth.'],
    ['Put a comma and a period.', 'Put a comma and a period.'],
    ['The word comma means a pause.', 'The word comma means a pause.'],
    ['Stop at the adapter period', 'Stop at the adapter period.'],
    ['Add a comma and finish period', 'Add a comma and finish period.'],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });

  it('dictionary name path still works around lists', () => {
    expect(
      ruleCleanup('Invite Sam comma pria comma and Jordan period', priya),
    ).toBe('Invite Sam, Priya, and Jordan.');
  });
});
