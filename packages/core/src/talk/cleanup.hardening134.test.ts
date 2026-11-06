import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe('ADR 0250 four-word Cap restart phrase', () => {
  it('collapses mid-sentence four-word Cap restarts', () => {
    for (const [raw, want] of [
      [
        'Then we could just have We could just have our configuration',
        'Then we could just have our configuration.',
      ],
      [
        'I think we should just ship We should just ship tomorrow',
        'I think we should just ship tomorrow.',
      ],
      [
        'Can you please check Can you please check the logs',
        'Can you please check the logs?',
      ],
      [
        "and then I don't really know I don't really know why",
        "And then I don't really know why.",
      ],
      [
        'so we need to wait We need to wait a bit longer',
        'So we need to wait a bit longer.',
      ],
      [
        'maybe you want to try You want to try again soon',
        'Maybe you want to try again soon.',
      ],
      [
        'What did they say What did they say about the plan',
        'What did they say about the plan?',
      ],
      [
        'how do you want How do you want this laid out',
        'How do you want this laid out?',
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses both-lower, Cap-last, and non-equal forms', () => {
    for (const [raw, want] of [
      [
        'we could just have we could just have our configuration',
        'We could just have we could just have our configuration.',
      ],
      [
        'New York City Hall New York City Hall is big',
        'New York City Hall New York City Hall is big.',
      ],
      [
        'San Francisco Bay Area San Francisco Bay Area grows',
        'San Francisco Bay Area San Francisco Bay Area grows.',
      ],
      [
        'the starting deck size the starting deck size matters',
        'The starting deck size the starting deck size matters.',
      ],
      [
        'You have to just wait you have to just wait longer',
        'You have to just wait you have to just wait longer.',
      ],
      [
        'Los Angeles County South Los Angeles County South thrives',
        'Los Angeles County South Los Angeles County South thrives.',
      ],
      [
        "I don't know how I Don't know how to finish",
        "I don't know how I Don't know how to finish.",
      ],
      [
        'we could just have something We could just have later',
        'We could just have something We could just have later.',
      ],
      [
        "I don't know I don't know how to do that UI",
        "I don't know how to do that UI.",
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
