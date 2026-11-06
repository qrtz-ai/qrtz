//! ADR 0250 (echo cycle 134): four-word Cap-leading restart phrases collapse.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn restart_phrase_collapses_four_word_cap() {
    for (raw, want) in [
        (
            "Then we could just have We could just have our configuration",
            "Then we could just have our configuration.",
        ),
        (
            "I think we should just ship We should just ship tomorrow",
            "I think we should just ship tomorrow.",
        ),
        (
            "Can you please check Can you please check the logs",
            "Can you please check the logs?",
        ),
        (
            "and then I don't really know I don't really know why",
            "And then I don't really know why.",
        ),
        (
            "so we need to wait We need to wait a bit longer",
            "So we need to wait a bit longer.",
        ),
        (
            "maybe you want to try You want to try again soon",
            "Maybe you want to try again soon.",
        ),
        (
            "What did they say What did they say about the plan",
            "What did they say about the plan?",
        ),
        (
            "how do you want How do you want this laid out",
            "How do you want this laid out?",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn restart_phrase_0250_refusals_stay() {
    for (raw, want) in [
        (
            "we could just have we could just have our configuration",
            "We could just have we could just have our configuration.",
        ),
        (
            "New York City Hall New York City Hall is big",
            "New York City Hall New York City Hall is big.",
        ),
        (
            "San Francisco Bay Area San Francisco Bay Area grows",
            "San Francisco Bay Area San Francisco Bay Area grows.",
        ),
        (
            "the starting deck size the starting deck size matters",
            "The starting deck size the starting deck size matters.",
        ),
        (
            "You have to just wait you have to just wait longer",
            "You have to just wait you have to just wait longer.",
        ),
        (
            "Los Angeles County South Los Angeles County South thrives",
            "Los Angeles County South Los Angeles County South thrives.",
        ),
        (
            "I don't know how I Don't know how to finish",
            "I don't know how I Don't know how to finish.",
        ),
        (
            "we could just have something We could just have later",
            "We could just have something We could just have later.",
        ),
        // Prior ADR 0247 three-word still collapses.
        (
            "I don't know I don't know how to do that UI",
            "I don't know how to do that UI.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
