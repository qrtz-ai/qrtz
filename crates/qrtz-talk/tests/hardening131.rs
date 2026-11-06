//! ADR 0247 (echo cycle 131): Cap-leading two-word and three-word restart phrases collapse.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn restart_phrase_collapses_title_leading_two_word() {
    for (raw, want) in [
        (
            "Can you Can you host the prototype on a server",
            "Can you host the prototype on a server?",
        ),
        (
            "And then It would It would be ordered",
            "And then It would be ordered.",
        ),
        (
            "That is That is more of a symbolic representation",
            "That is more of a symbolic representation.",
        ),
        (
            "I don't I don't like the two energy ability",
            "I don't like the two energy ability.",
        ),
        (
            "Let's just Let's just make the feature flag",
            "Let's just make the feature flag.",
        ),
        (
            "So I guess I guess I'm wondering if there's a way",
            "So I guess I'm wondering if there's a way.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn restart_phrase_collapses_three_word() {
    for (raw, want) in [
        (
            "I don't know I don't know how to do that UI",
            "I don't know how to do that UI.",
        ),
        (
            "How's it going How's it going with the launch today",
            "How's it going with the launch today?",
        ),
        (
            "What are the What are the deciding factors on both sides",
            "What are the deciding factors on both sides?",
        ),
        (
            "with like how I want How I want this to go",
            "With like how I want this to go.",
        ),
        (
            "Please tell me what are the What are the next steps",
            "Please tell me what are the next steps.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn restart_phrase_0247_refusals_stay() {
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
            "New York City New York City is big enough",
            "New York City New York City is big enough.",
        ),
        (
            "You have to you have to change that",
            "You have to you have to change that.",
        ),
        (
            "that doesn't solve the starting deck the starting deck problem",
            "That doesn't solve the starting deck the starting deck problem.",
        ),
        (
            "Please scratch that Scratch that and send the file",
            "Please scratch that Scratch that and send the file.",
        ),
        ("just use it as a As a mock-up", "Just use it as a mock-up."),
        ("Make make sure it uses CSS", "Make make sure it uses CSS."),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
