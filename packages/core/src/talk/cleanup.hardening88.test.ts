// ADR 0241 (echo cycle 88): a comma that sits only before a sentence mark is dropped.
import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

const priya = [{ term: 'Priya', aliases: ['pria'] }];

describe('ADR 0241 dangling comma before a sentence mark goes', () => {
  it.each([
    ['Yeah, also, uh.', 'Yeah, also.'],
    ['Yeah, also, um.', 'Yeah, also.'],
    ['So, um. Next we go', 'So. Next we go.'],
    ['Okay, , . Next', 'Okay. Next.'],
    [
      'On the notch thing, um. Make that a component',
      'On the notch thing. Make that a component.',
    ],
    ['Wait, uh!', 'Wait!'],
    ['Really, hmm?', 'Really?'],
    ['Done, uh...', 'Done...'],
    ['Thanks. So, um. Can you send it', 'Thanks. So. Can you send it?'],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });
});

describe('ADR 0241 comma before a mark stuck to a word stays', () => {
  it.each([
    ['Yes, .NET rocks on the server', 'Yes, .NET rocks on the server.'],
    [
      'The bump is 1, .5 more than last time',
      'The bump is 1, .5 more than last time.',
    ],
    ['Ship file, .pdf later today', 'Ship file, .pdf later today.'],
    ['Call it version 2, .0 for now', 'Call it version 2, .0 for now.'],
    ['The demo is, um, basically done.', 'The demo is basically done.'],
    ['So , um , we should wait', 'So, we should wait.'],
    ['Thanks. Um. Can you send it', 'Thanks. Can you send it?'],
    ['Thanks. Um, can you send it', 'Thanks. Can you send it?'],
    ['Over 1,000 people signed up.', 'Over 1,000 people signed up.'],
    ['Well, well, look who it is.', 'Well, well, look who it is.'],
  ])('%j -> %j', (raw, want) => {
    expect(ruleCleanup(raw)).toBe(want);
  });

  it('dictionary name path still works', () => {
    expect(ruleCleanup('Hey Priya, uh. Are you free tomorrow', priya)).toBe(
      'Hey Priya. Are you free tomorrow?',
    );
  });
});
