//! ADR 0245 (echo cycle 128): a filler right after a sentence end with a comma (or with only blanks before the next
//! word) is dropped, and a lowercase next letter is capitalised so the sentence mark stays.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn sentence_initial_filler_clause_keeps_the_sentence_mark() {
    for (raw, want) in [
        ("Thanks. Um, can you send it", "Thanks. Can you send it?"),
        ("Thanks. Uh, can you send it", "Thanks. Can you send it?"),
        ("Done. Um, can you review it", "Done. Can you review it?"),
        (
            "Ship it Friday. Uh, also send the notes",
            "Ship it Friday. Also send the notes.",
        ),
        (
            "Yeah, it works, thanks. Um, next, can you make it quicker",
            "Yeah, it works, thanks. Next, can you make it quicker.",
        ),
        ("thanks. Uh it says loom", "Thanks. It says loom."),
        (
            "Yeah, a couple things that I want to consider here. Um the first thing is timing",
            "Yeah, a couple things that I want to consider here. The first thing is timing.",
        ),
        (
            "I did just Google us and the good news is that we come up pretty fast. Um bad news is that the logo is missing",
            "I did just Google us and the good news is that we come up pretty fast. Bad news is that the logo is missing.",
        ),
        (
            "Okay, contour went pretty well. Uh what's Valerie Health look like?",
            "Okay, contour went pretty well. What's Valerie Health look like?",
        ),
        (
            "Maybe we should set up a logging system. Um, we can do that entirely offline as well",
            "Maybe we should set up a logging system. We can do that entirely offline as well.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn sentence_initial_filler_clause_refusals_stay() {
    for (raw, want) in [
        (
            "Grab the umbrella. Umbrella stands are by the door.",
            "Grab the umbrella. Umbrella stands are by the door.",
        ),
        (
            "Done. Umami is the fifth taste.",
            "Done. Umami is the fifth taste.",
        ),
        (
            "Okay. Uhura is my favorite character.",
            "Okay. Uhura is my favorite character.",
        ),
        (
            "Which school did you pick? UM. Go Blue!",
            "Which school did you pick? UM. Go Blue!",
        ),
        ("It works. UM, Okay", "It works. UM, Okay."),
        ("We should, uh, ship it Friday.", "We should ship it Friday."),
        (
            "So, um, we should wait until Monday",
            "So, we should wait until Monday.",
        ),
        ("So, um. Next we go", "So. Next we go."),
        ("Thanks. Um. Can you send it", "Thanks. Can you send it?"),
        // ADR 0235 still refuses lowercase after a filler's own mark.
        ("Thanks. Um. can you send it", "Thanks.. Can you send it?"),
        (
            "Turn left. Ahead is the parking garage.",
            "Turn left. Ahead is the parking garage.",
        ),
        (
            "Over 1,000 people signed up last week.",
            "Over 1,000 people signed up last week.",
        ),
        ("Mr. and Mrs. Smith are here.", "Mr. and Mrs. Smith are here."),
        (
            "Visit qrtz.ai. The docs are there.",
            "Visit qrtz.ai. The docs are there.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
