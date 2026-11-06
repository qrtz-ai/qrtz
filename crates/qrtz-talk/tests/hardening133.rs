//! ADR 0249 (echo cycle 133): bare lowercase "it" before Cap "It's" collapses to the Cap form.
use qrtz_talk::cleanup::rule_cleanup;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn restart_phrase_collapses_it_before_cap_its() {
    for (raw, want) in [
        ("Like it It's very hit or miss", "Like It's very hit or miss."),
        ("I think it It's broken already", "I think It's broken already."),
        ("because it It's not ready", "Because It's not ready."),
        ("and then it It's fine", "And then It's fine."),
        ("yeah it It's the same bug", "Yeah It's the same bug."),
        ("so it It's done for now", "So It's done for now."),
        ("when it It's cold out", "When It's cold out."),
        ("maybe it It's too late", "Maybe It's too late."),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn restart_phrase_0249_refusals_stay() {
    for (raw, want) in [
        ("it it's not a big deal", "It it's not a big deal."),
        ("with it it's hard to tell", "With it it's hard to tell."),
        ("about it it's fine by me", "About it it's fine by me."),
        ("It It's fine at the start", "It It's fine at the start."),
        ("give it its due today", "Give it its due today."),
        ("call it it and move on", "Call it and move on."),
        ("hit It's the limit now", "Hit It's the limit now."),
        ("bit It's weird today", "Bit It's weird today."),
        ("sit It's down carefully", "Sit It's down carefully."),
        ("it Itself is fine here", "It Itself is fine here."),
        ("from it it's clear enough", "From it it's clear enough."),
        ("after it it's done", "After it it's done."),
        // ADR 0248 equal-contraction still collapses.
        (
            "Yeah, it's still flying in, it's It's not correct",
            "Yeah, it's still flying in, it's not correct.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}
