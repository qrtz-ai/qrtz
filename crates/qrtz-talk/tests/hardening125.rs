//! ADR 0242 (echo cycle 125): spoken "comma" / "period" in an unambiguous list become "," / ".".
use qrtz_talk::cleanup::rule_cleanup;
use qrtz_talk::DictionaryEntry;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn spoken_list_comma_and_period_become_marks() {
    for (raw, want) in [
        (
            "Bring the charger comma the cable comma and the adapter period",
            "Bring the charger, the cable, and the adapter.",
        ),
        (
            "We need eggs comma milk comma and bread period",
            "We need eggs, milk, and bread.",
        ),
        (
            "Bring the charger, the cable, and the adapter period.",
            "Bring the charger, the cable, and the adapter.",
        ),
        (
            "Grab pens comma paper comma and tape period",
            "Grab pens, paper, and tape.",
        ),
        (
            "Pack socks comma shirts comma and a jacket period",
            "Pack socks, shirts, and a jacket.",
        ),
        (
            "Invite Sam comma Priya comma and Jordan period",
            "Invite Sam, Priya, and Jordan.",
        ),
        (
            "Buy milk comma eggs comma and bread period.",
            "Buy milk, eggs, and bread.",
        ),
        (
            "Notes: apples comma oranges comma and pears period",
            "Notes: apples, oranges, and pears.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn spoken_comma_and_period_as_nouns_stay() {
    for (raw, want) in [
        ("Add a comma after the name.", "Add a comma after the name."),
        ("The trial period ends Friday.", "The trial period ends Friday."),
        ("This is a period of growth.", "This is a period of growth."),
        (
            "Is that a question mark or a period",
            "Is that a question mark or a period.",
        ),
        (
            "Use a semicolon; then a comma, then a period.",
            "Use a semicolon; then a comma, then a period.",
        ),
        ("Put a comma and a period.", "Put a comma and a period."),
        ("The word comma means a pause.", "The word comma means a pause."),
        (
            "Please type comma and then continue.",
            "Please type comma and then continue.",
        ),
        (
            "We talked about the cooling-off period.",
            "We talked about the cooling-off period.",
        ),
        ("Grace period ends next week.", "Grace period ends next week."),
        ("Say the word comma and stop.", "Say the word comma and stop."),
        // No Oxford " comma and " and no ASCII comma → leave trailing spoken period alone.
        ("Stop at the adapter period", "Stop at the adapter period."),
        // Determiner before period with list commas still refuses.
        (
            "Bring eggs, milk, and a period",
            "Bring eggs, milk, and a period.",
        ),
        // "a comma" refuses conversion (converted == 0) so the whole rule no-ops, including trailing "period".
        ("Add a comma and finish period", "Add a comma and finish period."),
        (
            "Write the word comma and stop period",
            "Write the word comma and stop period.",
        ),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn dictionary_terms_still_apply_around_lists() {
    let dict = [DictionaryEntry {
        term: "Priya".into(),
        aliases: vec!["pria".into()],
    }];
    assert_eq!(
        rule_cleanup("Invite Sam comma pria comma and Jordan period", &dict),
        "Invite Sam, Priya, and Jordan."
    );
}
