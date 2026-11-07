import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0260 Cap Cap I across a period', () => {
  it('collapses', () => {
    const cases: Array<[string, string]> = [
      [
        'But I. I just feel like a moron today',
        'But I just feel like a moron today.',
      ],
      [
        'I. I think that works for the team today',
        'I think that works for the team today.',
      ],
      [
        'Yeah I. I agree with the privacy review today',
        'Yeah I agree with the privacy review today.',
      ],
      [
        "Oh yeah, I. I wasn't even thinking about that today",
        "Oh yeah, I wasn't even thinking about that today.",
      ],
      [
        'Next plus plus, I. I assume here this is something today',
        'Next plus plus, I assume here this is something today.',
      ],
      [
        "Like I. I don't know how to do all this stuff today",
        "Like I don't know how to do all this stuff today.",
      ],
      [
        'Because I. I absolutely forget all of those things today',
        'Because I absolutely forget all of those things today.',
      ],
      [
        'Sorry I. I actually want the card mechanics today',
        'Sorry I actually want the card mechanics today.',
      ],
      [
        "Like, I'm. I'm a senior engineer now today",
        "Like, I'm a senior engineer now today.",
      ],
      [
        "I don't. I don't know how to do all this stuff today",
        "I don't know how to do all this stuff today.",
      ],
    ];
    for (const [raw, want] of cases) {
      expect(ruleCleanup(raw, []), raw).toBe(want);
    }
  });

  it('refusals stay', () => {
    const cases: Array<[string, string]> = [
      [
        'So. So we should ship the flag today',
        'So. So we should ship the flag today.',
      ],
      [
        'Yes. Yes I can do that with the team today',
        'Yes. Yes I can do that with the team today.',
      ],
      [
        'Where. Where is the release for the team today',
        'Where. Where is the release for the team today?',
      ],
      [
        'What. What is the release for the team today',
        'What. What is the release for the team today?',
      ],
      [
        'The. The plan for the team is ready today',
        'The. The plan for the team is ready today.',
      ],
      [
        'Will. Will you join the standup with us today',
        'Will. Will you join the standup with us today?',
      ],
      [
        'We. We should ship the flag with the team today',
        'We. We should ship the flag with the team today.',
      ],
      [
        'A. A few days later we ship the flag today',
        'A. A few days later we ship the flag today.',
      ],
      [
        'I... I think that works for the team today',
        'I... I think that works for the team today.',
      ],
      [
        'I talk. I talk like I know everything today',
        'I talk. I talk like I know everything today.',
      ],
      [
        "I'm I'm a senior engineer on the team today",
        "I'm I'm a senior engineer on the team today.",
      ],
    ];
    for (const [raw, want] of cases) {
      expect(ruleCleanup(raw, []), raw).toBe(want);
    }
  });
});
