//! ADR 0241 (echo cycle 88): a comma that sits only before a sentence mark is dropped
//! ("Yeah, also, uh." -> "Yeah, also."; "So, um. Next we go" -> "So. Next we go.").
use qrtz_talk::cleanup::rule_cleanup;
use qrtz_talk::DictionaryEntry;

fn clean(raw: &str) -> String {
    rule_cleanup(raw, &[])
}

#[test]
fn a_dangling_comma_before_a_sentence_mark_goes() {
    for (raw, want) in [
        ("Yeah, also, uh.", "Yeah, also."),
        ("Yeah, also, um.", "Yeah, also."),
        ("So, um. Next we go", "So. Next we go."),
        ("Okay, , . Next", "Okay. Next."),
        (
            "On the notch thing, um. Make that a component",
            "On the notch thing. Make that a component.",
        ),
        ("Wait, uh!", "Wait!"),
        ("Really, hmm?", "Really?"),
        ("Done, uh...", "Done..."),
        ("Thanks. So, um. Can you send it", "Thanks. So. Can you send it?"),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn a_comma_before_a_mark_stuck_to_a_word_stays() {
    for (raw, want) in [
        ("Yes, .NET rocks on the server", "Yes, .NET rocks on the server."),
        (
            "The bump is 1, .5 more than last time",
            "The bump is 1, .5 more than last time.",
        ),
        ("Ship file, .pdf later today", "Ship file, .pdf later today."),
        ("Call it version 2, .0 for now", "Call it version 2, .0 for now."),
        // Normal commas and fillers that take their comma stay as before.
        ("The demo is, um, basically done.", "The demo is basically done."),
        ("So , um , we should wait", "So, we should wait."),
        ("Thanks. Um. Can you send it", "Thanks. Can you send it?"),
        ("Thanks. Um, can you send it", "Thanks can you send it."),
        ("Over 1,000 people signed up.", "Over 1,000 people signed up."),
        ("Well, well, look who it is.", "Well, well, look who it is."),
    ] {
        assert_eq!(clean(raw), want, "{raw:?}");
    }
}

#[test]
fn dictionary_terms_are_untouched_by_the_scan() {
    // The step does not read the dictionary; a term that happens to hold ", ." is still
    // rewritten like any other text (no opt-out). Pin the ordinary path with a name.
    let dict = [DictionaryEntry {
        term: "Priya".into(),
        aliases: vec!["pria".into()],
    }];
    assert_eq!(
        rule_cleanup("Hey Priya, uh. Are you free tomorrow", &dict),
        "Hey Priya. Are you free tomorrow?"
    );
}
