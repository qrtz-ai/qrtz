import { describe, expect, it } from 'vitest';
import { ruleCleanup } from './cleanup';

describe("ADR 0249 bare it before Cap It's restart", () => {
  it("collapses mid-sentence it before Cap It's", () => {
    for (const [raw, want] of [
      ["Like it It's very hit or miss", "Like It's very hit or miss."],
      ["I think it It's broken already", "I think It's broken already."],
      ["because it It's not ready", "Because It's not ready."],
      ["and then it It's fine", "And then It's fine."],
      ["yeah it It's the same bug", "Yeah It's the same bug."],
      ["so it It's done for now", "So It's done for now."],
      ["when it It's cold out", "When It's cold out."],
      ["maybe it It's too late", "Maybe It's too late."],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });

  it('refuses both-lower, Cap-It, and non-exact forms', () => {
    for (const [raw, want] of [
      ["it it's not a big deal", "It it's not a big deal."],
      ["with it it's hard to tell", "With it it's hard to tell."],
      ["about it it's fine by me", "About it it's fine by me."],
      ["It It's fine at the start", "It It's fine at the start."],
      ['give it its due today', 'Give it its due today.'],
      ['call it it and move on', 'Call it and move on.'],
      ["hit It's the limit now", "Hit It's the limit now."],
      ["bit It's weird today", "Bit It's weird today."],
      ["sit It's down carefully", "Sit It's down carefully."],
      ['it Itself is fine here', 'It Itself is fine here.'],
      ["from it it's clear enough", "From it it's clear enough."],
      ["after it it's done", "After it it's done."],
      [
        "Yeah, it's still flying in, it's It's not correct",
        "Yeah, it's still flying in, it's not correct.",
      ],
    ] as const) {
      expect(ruleCleanup(raw)).toBe(want);
    }
  });
});
