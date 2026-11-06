//! ADR 0246 (echo cycle 130): a mid-sentence two-word restart phrase collapses to one copy.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn restart_phrase_collapses_lowercase_then_title_lower() {
    for (raw, want) in [
        (
            "just use it as a As a mock-up, but do the full architecture",
            "Just use it as a mock-up, but do the full architecture.",
        ),
        (
            "can you do so in a In a way that doesn't interfere",
            "Can you do so in a way that doesn't interfere.",
        ),
        (
            "complexity in either insertion or retrieval and in the In the best interest of maintaining all the data",
            "Complexity in either insertion or retrieval and in the best interest of maintaining all the data.",
        ),
        (
            "To do the next thing, it doesn't It doesn't quite do it",
            "To do the next thing, it doesn't quite do it.",
        ),
        (
            "Please render it as a As a full-screen card",
            "Please render it as a full-screen card.",
        ),
        (
            "Ship the change in a In a separate PR",
            "Ship the change in a separate PR.",
        ),
        (
            "Keep the rows in the In the original order",
            "Keep the rows in the original order.",
        ),
        (
            "After the click it doesn't It doesn't fire twice",
            "After the click it doesn't fire twice.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn restart_phrase_refusals_stay() {
    for (raw, want) in [
        (
            "We moved to new york New York last year",
            "We moved to new york New York last year.",
        ),
        (
            "Open the app store App Store on your phone",
            "Open the app store App Store on your phone.",
        ),
        (
            "He flew to los angeles Los Angeles this morning",
            "He flew to los angeles Los Angeles this morning.",
        ),
        (
            "Visit san francisco San Francisco next week",
            "Visit san francisco San Francisco next week.",
        ),
        (
            "I studied machine learning Machine Learning in college",
            "I studied machine learning Machine Learning in college.",
        ),
        (
            "She works for united states United States postal service",
            "She works for united states United States postal service.",
        ),
        (
            "Why do we have assets and product images? Product images is a subset",
            "Why do we have assets and product images? Product images is a subset.",
        ),
        ("Make make sure it uses CSS", "Make make sure it uses CSS."),
        ("Thanks. Um, can you send it", "Thanks. Can you send it?"),
        ("We should, uh, ship it Friday.", "We should ship it Friday."),
        ("I I think the the plan works.", "I think the plan works."),
        ("That works for me, right", "That works for me, right?"),
        (
            "Please scratch that\nScratch that and send the file",
            "Please scratch that\nScratch that and send the file.",
        ),
        (
            "Please scratch that Scratch that and send the file",
            "Please scratch that Scratch that and send the file.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
