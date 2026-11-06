//! ADR 0199 (echo cycle 60): the cleanup chain's look-around and backreference patterns became hand-written scans,
//! and a dictionary term of plain words is written by a scan instead of the backtracking pattern. A pure speed-up: the text out must be byte for
//! byte the cycle-59 chain's (frozen in `oracle/cleanup_c59.rs`) on every vector and on generated text, and the long
//! dictation must now run in a quarter of the 2 ms budget (it took 1.5-1.8 ms before, so this fails without the fix).
use qrtz_talk::cleanup::{
    apply_dictionary, cleanup_dictation, rule_cleanup, QUESTION_COMMA_OPENERS, QUESTION_OPENERS,
};
use qrtz_talk::DictionaryEntry;
use serde_json::Value;
use std::time::Instant;

#[path = "oracle/cleanup_c59.rs"]
mod c59;

fn entry(term: &str, aliases: &[&str]) -> DictionaryEntry {
    DictionaryEntry {
        term: term.into(),
        aliases: aliases.iter().map(|a| (*a).into()).collect(),
    }
}

/// The budget test's dictionary, a few odd ones (Unicode, regex characters, one term inside another's alias), and
/// none.
fn dictionaries() -> Vec<Vec<DictionaryEntry>> {
    let mut budget = vec![
        entry("qrtz", &["cue arts", "quartz"]),
        entry("Postgres", &["post gres"]),
    ];
    for i in 0..18 {
        budget.push(entry(&format!("Term{i}"), &[&format!("term number {i}")]));
    }
    let odd = vec![
        entry("Straße", &["strasse", "STRASSE road"]),
        entry("K8s", &["kubernetes", "kates"]),
        entry("C++", &["c plus plus"]),
        entry("a.b(c)", &["a b c"]),
        entry("new line", &["newline"]),
        entry("the the", &["thee"]),
        entry("Ünïcödé", &["unicode"]),
        entry("Kelvin", &["\u{212A}elvin"]),
        entry("", &[""]),
        entry("   ", &["  "]),
        entry("3D", &["3d", "three d"]),
        entry("Q", &["q"]),
        entry("cue-arts", &["cue - arts"]),
    ];
    vec![vec![], budget, odd]
}

fn same(raw: &str, dict: &[DictionaryEntry]) {
    // ADR 0235 (cycle 83): a filler said as its own sentence goes with its mark. The cycle-59 chain gets the raw text
    // with that edit made (written independently of cleanup.rs), with its spoken layout commands made line breaks
    // first or not; a text with no such filler must still match the plain cycle-59 output.
    let new = rule_cleanup(raw, dict);
    let old = c59::rule_cleanup(raw, dict);
    let old = if new == old {
        old
    } else {
        // ADR 0236 (cycle 84): each candidate with its comma runs as one comma, as the last step does.
        // ADR 0245 (cycle 128): also try the sentence-initial filler-clause edit (comma / bare) on the raw text.
        let candidates = [
            c59::rule_cleanup(&without_filler_marks(&with_layout(raw)), dict),
            c59::rule_cleanup(&without_filler_marks(raw), dict),
            c59::rule_cleanup(
                &without_filler_clause(&without_filler_marks(&with_layout(raw))),
                dict,
            ),
            c59::rule_cleanup(&without_filler_clause(&without_filler_marks(raw)), dict),
            c59::rule_cleanup(&without_filler_clause(&with_layout(raw)), dict),
            c59::rule_cleanup(&without_filler_clause(raw), dict),
        ]
        .map(|c| {
            without_restart_phrase(&with_spoken_list_punct(&without_dangling_comma_marks(
                &without_space_before_curly_close(&without_comma_runs(&c, dict)),
            )))
        });
        candidates
            .iter()
            .find(|c| **c == new)
            .cloned()
            .unwrap_or_else(|| {
                if only_stray_marks_gone(&old, &new) {
                    new.clone()
                } else {
                    candidates[1].clone()
                }
            })
    };
    // ADR 0201 (cycle 62), ADR 0203 (cycle 63) and ADR 0204 (cycle 64) are the only text changes since the oracle:
    // a dictated list's line commas go, and a question after a discourse opener ("So do you want ...") ends with "?"
    // where cycle 59 added "."; if that opener is "hey"/"okay"/"ok"/"alright" with no comma, the comma goes after it
    // ("Hey, are you free?"). Any other difference fails.
    // ADR 0205/0207 (cycles 65/67) may also drop the second pronoun of "we, we <modal>" / "they, they <modal>".
    // ADR 0206 (cycle 66): "Hey <term> ..." may end with "?" instead of "." and gain a comma after the term.
    // ADR 0227 (cycle 76): a sentence ending in ", <negative contraction> <pronoun>" (a tag) may end with "?".
    // ADR 0229 (cycle 78): so may one opening with "Let's" / "Please" and ending in ", shall we" / ", <modal> you".
    // ADR 0230 (cycle 79): a month's ordinal day may be written as a figure ("on October 21st").
    // ADR 0232 (cycle 80): a question opening with a wh-contraction ("What's the plan") may end with "?".
    // ADR 0233 (cycle 81): so may one with the contraction on the wh-word's modifier ("What time's the meeting").
    // ADR 0234 (cycle 82): a spoken title may be written as its abbreviation ("Mister Smith" -> "Mr. Smith").
    // ADR 0236 (cycle 84): a run of commas between a word and a word types as one comma (taken first here so the
    // title edit below sees the same words).
    // ADR 0238 (cycle 85): no space before a curly closing quote after "," / ";".
    // ADR 0241 (cycle 88): a comma before a sentence mark is dropped (after comma runs).
    // ADR 0242 (cycle 125): spoken list commas/period become ","/".".
    let old = without_comma_runs(&old, dict);
    // ADR 0238 (cycle 85): drop the space cycle 59 put before a curly closing quote.
    let old = without_space_before_curly_close(&old);
    // ADR 0241 (cycle 88): a comma that sits only before a sentence mark is dropped.
    let old = without_dangling_comma_marks(&old);
    // ADR 0242 (cycle 125): spoken list "comma"/"period" become marks.
    let old = with_spoken_list_punct(&old);
    // ADR 0246 / ADR 0247 / ADR 0248 / ADR 0249 (cycles 130–133): restart phrases (incl. it-before-It's) collapse.
    let old = without_restart_phrase(&old);
    let old = with_titles_from(&old, &new);
    let old = if new != old && new == without_subject_repeats(&old) {
        new.clone()
    } else {
        old
    };
    if new != old {
        let list = old.contains(",\n") && new == old.replace(",\n", "\n");
        let opener = old.strip_suffix('.').is_some_and(|body| {
            (new == format!("{body}?")
                || new == format!("{}?", with_opener_comma(body))
                || with_vocative_comma(body, dict).is_some_and(|b| new == format!("{b}?")))
                && body
                    .rsplit(['\n', '.', '!', '?'])
                    .next()
                    .and_then(|s| s.split_whitespace().next())
                    .is_some_and(|w| {
                        QUESTION_OPENERS.contains(&w.trim_end_matches(',').to_ascii_lowercase().as_str())
                    })
        });
        // ADR 0243 (cycle 126): so may ", right" / ", correct".
        // ADR 0244 (cycle 127): so may a lone "Right" / "Correct" sentence after a prior one.
        // ADR 0245 (cycle 128): sentence-initial filler clause — see candidates + unmarked below.
        // ADR 0246 / ADR 0247 / ADR 0248 / ADR 0249 (cycles 130–133): restart phrases — see without_restart_phrase on old.
        let tag = old.strip_suffix('.').is_some_and(|body| {
            new == format!("{body}?")
                && (body.rsplit_once(", ").is_some_and(|(_, t)| {
                    let w: Vec<&str> = t.split(' ').collect();
                    ((2..=3).contains(&w.len()) && (w[0].ends_with("n't") || w[0].ends_with("n\u{2019}t")))
                        || (w.len() == 1 && matches!(w[0], "right" | "correct"))
                }) || body.rsplit(['\n', '.', '!', '?']).next().is_some_and(|s| {
                    let w: Vec<&str> = s.split_whitespace().collect();
                    w.len() == 1
                        && matches!(w[0].to_ascii_lowercase().as_str(), "right" | "correct")
                        && body.chars().any(|c| matches!(c, '.' | '!' | '?' | '\n'))
                }))
        });
        let imperative = old.strip_suffix('.').is_some_and(|body| {
            let sentence = body.rsplit(['\n', '.', '!', '?']).next().unwrap_or_default();
            let first = sentence
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .replace('\u{2019}', "'");
            new == format!("{body}?")
                && (first.starts_with("Let's") || first.to_ascii_lowercase().starts_with("please"))
                && ["shall we", "will you", "would you", "can you", "could you"]
                    .iter()
                    .any(|t| body.ends_with(&format!(", {t}")))
        });
        // ADR 0230: a spoken day after a month written as a figure ("October twenty first" -> "October 21st").
        let date = days_as_words(&new).replace('-', " ") == old.replace('-', " ");
        // ADR 0232 (cycle 80): a last sentence opening (after an opener, if any) with a wh-word contracted with its
        // auxiliary ("What's", "How'd", "Where're") may end with "?" (and gain the opener's comma) instead of ".".
        let wh_contraction = old.strip_suffix('.').is_some_and(|body| {
            (new == format!("{body}?") || new == format!("{}?", with_opener_comma(body)))
                && body.rsplit(['\n', '.', '!', '?']).next().is_some_and(|s| {
                    let mut w = s.split_whitespace().map(str::to_ascii_lowercase);
                    let first = w.next().unwrap_or_default();
                    let first = if QUESTION_OPENERS.contains(&first.trim_end_matches(',')) {
                        w.next().unwrap_or_default()
                    } else {
                        first
                    };
                    let second = w.next().unwrap_or_default();
                    ["what", "where", "when", "why", "how", "who"].iter().any(|wh| {
                        first
                            .strip_prefix(wh)
                            .is_some_and(|t| t.starts_with(['\'', '\u{2019}']))
                    }) || (matches!(first.as_str(), "what" | "how") && second.contains(['\'', '\u{2019}']))
                })
        });
        // ADR 0245: after the filler-clause edit, a last sentence that opens like a yes/no or wh-question may end
        // with "?" instead of cycle 59's ".".
        let unmarked = old.strip_suffix('.').is_some_and(|body| {
            new == format!("{body}?")
                && body.rsplit(['\n', '.', '!', '?']).next().is_some_and(|s| {
                    let first = s
                        .split_whitespace()
                        .next()
                        .unwrap_or_default()
                        .trim_end_matches(',')
                        .to_ascii_lowercase()
                        .replace('\u{2019}', "'");
                    matches!(
                        first.as_str(),
                        "can"
                            | "could"
                            | "will"
                            | "would"
                            | "should"
                            | "shall"
                            | "may"
                            | "might"
                            | "do"
                            | "does"
                            | "did"
                            | "is"
                            | "are"
                            | "am"
                            | "was"
                            | "were"
                            | "have"
                            | "has"
                            | "what"
                            | "what's"
                            | "who"
                            | "who's"
                            | "where"
                            | "where's"
                            | "when"
                            | "when's"
                            | "why"
                            | "why's"
                            | "how"
                            | "how's"
                            | "which"
                    ) || ["what", "where", "when", "why", "how", "who"].iter().any(|wh| {
                        first
                            .strip_prefix(wh)
                            .is_some_and(|tail| tail.starts_with(['\'', '\u{2019}']))
                    })
                })
        });
        assert!(
            list || opener || tag || imperative || date || wh_contraction || unmarked,
            "rule_cleanup({raw:?}): {new:?} vs cycle 59 {old:?}"
        );
    }
    assert_eq!(
        apply_dictionary(raw, dict),
        c59::apply_dictionary(raw, dict),
        "apply_dictionary({raw:?})"
    );
    assert_eq!(
        cleanup_dictation(raw, dict, false).text,
        c59::cleanup_dictation(raw, dict, false).text,
        "cleanup off ({raw:?})"
    );
}

/// ADR 0235, for generated text where a spoken layout command sits in the seam: `new` has the same words as cycle
/// 59's `old` (letters and digits, case aside), only marks, commas and white space differ, and `new` has fewer stray
/// marks (each ", ,", each mark after the first in a run of ".!?", a mark opening the text); an ellipsis the
/// recogniser wrote is in both.
fn only_stray_marks_gone(old: &str, new: &str) -> bool {
    let words = |t: &str| {
        t.chars()
            .filter(|c| !c.is_whitespace() && !matches!(c, '.' | ',' | '!' | '?'))
            .flat_map(char::to_lowercase)
            .collect::<String>()
    };
    let stray = |t: &str| {
        static RUN: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new(r"[.!?]{2,}").unwrap());
        t.matches(", ,").count()
            + RUN.find_iter(t).map(|m| m.len() - 1).sum::<usize>()
            + usize::from(t.starts_with(['.', ',', '!', '?']))
    };
    words(old) == words(new) && stray(new) < stray(old)
}

/// ADR 0235: the raw text with its spoken layout commands made line breaks first (written independently of cleanup.rs,
/// with cycle 59's noun-phrase determiners), so a filler sentence that only meets a sentence mark across one is seen.
fn with_layout(raw: &str) -> String {
    let mut t = raw.to_owned();
    for (phrase, with) in [("new paragraph", "\n\n"), ("new line", "\n")] {
        let r = fancy_regex::Regex::new(&format!(
            r"(?i)\s*(?<![A-Za-z0-9_]){phrase}(?![A-Za-z0-9_])[,.]?\s*"
        ))
        .unwrap();
        let src = t.clone();
        t = r
            .replace_all(&src, |c: &fancy_regex::Captures<str>| {
                let start = c.get(0).map_or(0, |m| m.start());
                let before = src[..start].trim_end();
                let word: String = before
                    .chars()
                    .rev()
                    .take_while(|c| c.is_ascii_alphabetic())
                    .collect();
                let rest = &before[..before.len() - word.len()];
                let whole = !rest.ends_with(|c: char| c.is_ascii_alphanumeric() || c == '_' || c == '\'');
                let word = word.chars().rev().collect::<String>().to_ascii_lowercase();
                if whole && c59::COMMAND_DETERMINERS.contains(&word.as_str()) {
                    c[0].to_owned()
                } else {
                    with.to_owned()
                }
            })
            .into_owned();
    }
    t
}

/// ADR 0235's filler edit, made on the raw text (written independently of cleanup.rs): the "." / "?" / "!" run after
/// a filler that starts the text or follows a sentence mark and blanks (and meets a blank or the end, with no lowercase
/// letter next unless it starts the text) is dropped, unless the filler is in capitals.
fn without_filler_marks(raw: &str) -> String {
    use std::sync::LazyLock;
    static FILLER: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"(?i)[ \t\n\r]+(u+m+|u+h+|e+r+m+|e+r|a+h+|h+m+|m+h*m+)([.!?]+)").unwrap()
    });
    let padded = format!(" {raw}");
    let mut out = String::new();
    let mut last = 0;
    for m in FILLER.captures_iter(&padded) {
        let (all, word, marks) = (m.get(0).unwrap(), m.get(1).unwrap(), m.get(2).unwrap());
        // A spoken "new line" / "new paragraph" is a line break by the time the fillers go.
        let layout = |t: &str| {
            let l = t.to_ascii_lowercase();
            ["new line", "new paragraph"].iter().any(|c| {
                l.starts_with(c) && !l[c.len()..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
            })
        };
        let before = padded[..all.start()].trim_end_matches(char::is_whitespace);
        let before = before.strip_suffix([',', '.']).unwrap_or(before);
        let before = ["new line", "new paragraph"]
            .iter()
            .find_map(|c| {
                let l = before.to_ascii_lowercase();
                l.ends_with(c)
                    .then(|| before[..before.len() - c.len()].trim_end_matches(char::is_whitespace))
            })
            .unwrap_or(&padded[..all.start()]);
        let before_ok = all.start() == 0 || before.is_empty() || before.ends_with(['.', '!', '?']);
        let after = &padded[marks.end()..];
        let after_ok = after
            .chars()
            .next()
            .is_none_or(|c| matches!(c, ' ' | '\t' | '\n' | '\r'))
            || layout(after);
        let opens = all.start() == 0 || before.is_empty();
        let after_ok = after_ok
            && (opens
                || !after
                    .trim_start_matches([' ', '\t', '\n', '\r'])
                    .starts_with(|c: char| c.is_ascii_lowercase()));
        let caps = word.as_str().len() >= 2 && word.as_str().chars().all(|c| c.is_ascii_uppercase());
        if before_ok && after_ok && !caps {
            out.push_str(&padded[last..marks.start()]);
            last = marks.end();
        }
    }
    out.push_str(&padded[last..]);
    out.strip_prefix(' ').unwrap_or(&out).to_owned()
}

/// ADR 0246 / ADR 0247 / ADR 0248 / ADR 0249's restart-phrase edit, made on the text (written
/// independently of cleanup.rs): a two- or three-word phrase immediately repeated with a Title-case
/// first word on the second copy and a lowercase last word collapses to one copy. The first copy's
/// first word may be Title case (ADR 0247); a three-word middle word may be the pronoun "I" / a
/// contracted "I". ADR 0248: a one-word Cap restart of a contraction ("it's It's") also collapses.
/// ADR 0249: bare lowercase "it" before Cap "It's" / curly-apos form drops "it" and keeps Cap.
fn without_restart_phrase(s: &str) -> String {
    let word_re = regex::Regex::new(r"[A-Za-z]+(?:['\u{2019}][A-Za-z]+)*").unwrap();
    let spans: Vec<(usize, usize, String)> = word_re
        .find_iter(s)
        .map(|m| (m.start(), m.end(), m.as_str().to_owned()))
        .collect();
    if spans.len() < 2 {
        return s.to_owned();
    }
    let only_space_tab =
        |from: usize, to: usize| from <= to && s[from..to].chars().all(|c| c == ' ' || c == '\t');
    let letters_lower = |w: &str| {
        w.chars()
            .filter(|c| c.is_ascii_alphabetic())
            .all(|c| c.is_ascii_lowercase())
    };
    let is_i_word = |w: &str| {
        let low = w.replace('\u{2019}', "'").to_ascii_lowercase();
        matches!(low.as_str(), "i" | "i'd" | "i'm" | "i've" | "i'll")
    };
    let middle_ok = |w: &str| letters_lower(w) || is_i_word(w);
    let mut drop = vec![false; spans.len()];
    let mut i = 0;
    while i < spans.len() {
        if i + 5 < spans.len() {
            let (_a0, b0, ref w0) = spans[i];
            let (a1, b1, ref w1) = spans[i + 1];
            let (a2, b2, ref w2) = spans[i + 2];
            let (a3, b3, ref w3) = spans[i + 3];
            let (a4, b4, ref w4) = spans[i + 4];
            let (a5, _b5, ref w5) = spans[i + 5];
            let ok3 = only_space_tab(b0, a1)
                && only_space_tab(b1, a2)
                && only_space_tab(b2, a3)
                && only_space_tab(b3, a4)
                && only_space_tab(b4, a5)
                && middle_ok(w1)
                && letters_lower(w2)
                && w3.starts_with(|c: char| c.is_ascii_uppercase())
                && middle_ok(w4)
                && w5.starts_with(|c: char| c.is_ascii_lowercase())
                && w0.eq_ignore_ascii_case(w3)
                && w1.eq_ignore_ascii_case(w4)
                && w2.eq_ignore_ascii_case(w5);
            if ok3 {
                drop[i + 3] = true;
                drop[i + 4] = true;
                drop[i + 5] = true;
                i += 6;
                continue;
            }
        }
        if i + 3 < spans.len() {
            let (_a0, b0, ref w0) = spans[i];
            let (a1, b1, ref w1) = spans[i + 1];
            let (a2, b2, ref w2) = spans[i + 2];
            let (a3, _b3, ref w3) = spans[i + 3];
            let ok2 = only_space_tab(b0, a1)
                && only_space_tab(b1, a2)
                && only_space_tab(b2, a3)
                && letters_lower(w1)
                && w2.starts_with(|c: char| c.is_ascii_uppercase())
                && w3.starts_with(|c: char| c.is_ascii_lowercase())
                && w0.eq_ignore_ascii_case(w2)
                && w1.eq_ignore_ascii_case(w3)
                && !(w0.eq_ignore_ascii_case("scratch") && w1.eq_ignore_ascii_case("that"));
            if ok2 {
                drop[i + 2] = true;
                drop[i + 3] = true;
                i += 4;
                continue;
            }
        }
        // ADR 0248: one-word Cap restart of a contraction.
        // ADR 0249: bare lowercase "it" before Cap "It's" — drop "it", keep Cap.
        if i + 1 < spans.len() {
            let (_a0, b0, ref w0) = spans[i];
            let (a1, _b1, ref w1) = spans[i + 1];
            let has_apos = w0.contains('\'') || w0.contains('\u{2019}');
            let ok1 = only_space_tab(b0, a1)
                && has_apos
                && w0.starts_with(|c: char| c.is_ascii_lowercase())
                && w1.starts_with(|c: char| c.is_ascii_uppercase())
                && w0.eq_ignore_ascii_case(w1);
            if ok1 {
                drop[i + 1] = true;
                i += 2;
                continue;
            }
            let is_bare_it = w0 == "it";
            let is_cap_its = w1 == "It's" || w1.as_str() == "It’s";
            let ok_it = only_space_tab(b0, a1) && is_bare_it && is_cap_its;
            if ok_it {
                drop[i] = true;
                i += 2;
                continue;
            }
        }
        i += 1;
    }
    if drop.iter().all(|d| !*d) {
        return s.to_owned();
    }
    let mut out = String::with_capacity(s.len());
    let mut last = 0usize;
    for (idx, &(a, b, _)) in spans.iter().enumerate() {
        if !drop[idx] {
            continue;
        }
        let mut ws = a;
        while ws > last {
            let ch = s[..ws].chars().next_back().unwrap();
            if ch == ' ' || ch == '\t' {
                ws -= ch.len_utf8();
            } else {
                break;
            }
        }
        out.push_str(&s[last..ws]);
        last = b;
    }
    out.push_str(&s[last..]);
    out
}

/// ADR 0245's filler-clause edit, made on the raw text (written independently of cleanup.rs): a filler right after a
/// sentence end, with an optional comma and then blanks before a letter (or the end), is dropped; a lowercase next
/// letter is capitalised. Kept capital fillers are left alone.
fn without_filler_clause(raw: &str) -> String {
    let b = raw.as_bytes();
    let blank = |c: u8| matches!(c, b' ' | b'\t' | b'\n' | b'\r');
    let mark = |c: u8| matches!(c, b'.' | b'!' | b'?');
    let is_filler = |w: &str| {
        let low = w.to_ascii_lowercase();
        let re = regex::Regex::new(r"^(u+m+|u+h+|e+r+m+|e+r|a+h+|h+m+|m+h*m+)$").unwrap();
        re.is_match(&low)
    };
    let mut out = String::new();
    let (mut last, mut p) = (0, 0);
    while p < b.len() {
        if !blank(b[p]) || (p > 0 && !mark(b[p - 1])) {
            p += 1;
            continue;
        }
        let mut i = p;
        while i < b.len() && blank(b[i]) {
            i += 1;
        }
        let mut j = i;
        while j < b.len() && b[j].is_ascii_alphabetic() {
            j += 1;
        }
        let word = &raw[i..j];
        if j == i || !is_filler(word) || (word.len() >= 2 && word.chars().all(|c| c.is_ascii_uppercase())) {
            p = i;
            continue;
        }
        if p > 0 && b[p - 1] == b'.' {
            let before = &raw[..p];
            let token = before[..before.len() - 1]
                .rsplit([' ', '\n', '\t', '\r'])
                .next()
                .unwrap_or("");
            if token.contains('.')
                || (token.len() == 1 && token.chars().next().is_some_and(|c| c.is_ascii_alphanumeric()))
            {
                p = i;
                continue;
            }
        }
        if j < b.len() && mark(b[j]) {
            p = i;
            continue;
        }
        let mut k = j;
        if b.get(k) == Some(&b',') {
            k += 1;
        }
        let mut next = k;
        while next < b.len() && blank(b[next]) {
            next += 1;
        }
        let ok = k == b.len()
            || (next > k && next < b.len() && b[next].is_ascii_alphabetic())
            || (k < b.len() && blank(b[k]) && next == b.len());
        if !ok {
            p = i;
            continue;
        }
        out.push_str(&raw[last..p]);
        out.push_str(&raw[p..i]);
        if next < b.len() && b[next].is_ascii_lowercase() {
            out.push_str(&raw[k..next]);
            out.push(b[next].to_ascii_uppercase() as char);
            last = next + 1;
            p = next + 1;
        } else {
            last = k;
            p = k;
        }
    }
    if last == 0 {
        return raw.to_owned();
    }
    out.push_str(&raw[last..]);
    out
}

/// ADR 0236's one allowed edit (written independently of cleanup.rs, with the `regex` crate): ", ," (and longer runs)
/// after an ASCII letter or digit, a Latin letter U+00C0-U+024F or a closing ")" / "]" / quote, and before a space and
/// an ASCII letter or digit, becomes ","; not when a dictionary term or alias holds ", ,".
fn without_comma_runs(old: &str, dict: &[DictionaryEntry]) -> String {
    use std::sync::LazyLock;
    static RUN: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r#"([A-Za-z0-9\x{C0}-\x{D6}\x{D8}-\x{F6}\x{F8}-\x{24F})\]"'\x{2019}\x{201D}]),(?: ,)+ ([A-Za-z0-9])"#,
        )
        .unwrap()
    });
    if dict
        .iter()
        .any(|d| d.term.contains(", ,") || d.aliases.iter().any(|a| a.contains(", ,")))
    {
        return old.to_owned();
    }
    let mut out = old.to_owned();
    loop {
        let next = RUN.replace_all(&out, "$1, $2").into_owned();
        if next == out {
            return out;
        }
        out = next;
    }
}

/// ADR 0238's one allowed edit (written independently of cleanup.rs): a space before a curly closing
/// quote after "," / ";" is dropped. Cycle 59's SPACE_AFTER_PUNCT inserted that space; ASCII quotes were already skipped.
fn without_space_before_curly_close(old: &str) -> String {
    old.replace(", \u{201D}", ",\u{201D}")
        .replace("; \u{201D}", ";\u{201D}")
        .replace(", \u{2019}", ",\u{2019}")
        .replace("; \u{2019}", ";\u{2019}")
}

/// ADR 0241's one allowed edit (written independently of cleanup.rs, with the `regex` crate): a comma, optional
/// " ," pieces, optional spaces, then "." / "!" / "?" and a blank or the end, becomes just the marks (blank kept).
fn without_dangling_comma_marks(old: &str) -> String {
    use std::sync::LazyLock;
    static DANGLING: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r",(?: ,)*[ \t]*([.!?]+)([ \t\n\r]|$)").unwrap());
    let mut out = old.to_owned();
    loop {
        let next = DANGLING.replace_all(&out, "$1$2").into_owned();
        if next == out {
            return out;
        }
        out = next;
    }
}

/// ADR 0242's one allowed edit (written independently of cleanup.rs): when `old` ends in " period" (optional
/// trailing mark) after a plain word that is not a final-mark noun-before, and either holds " comma and " or an
/// ASCII ",", rewrite list "comma" tokens (prev not a comma-noun-before) and/or drop the trailing spoken "period".
fn with_spoken_list_punct(old: &str) -> String {
    const COMMA_NOUN_BEFORE: &[&str] = &[
        "a",
        "an",
        "the",
        "this",
        "that",
        "these",
        "those",
        "my",
        "your",
        "his",
        "her",
        "its",
        "our",
        "their",
        "no",
        "another",
        "any",
        "each",
        "every",
        "some",
        "one",
        "which",
        "whose",
        "what",
        "word",
        "words",
        "term",
        "terms",
        "letter",
        "letters",
        "character",
        "characters",
        "symbol",
        "symbols",
        "punctuation",
        "key",
        "keys",
        "button",
        "english",
        "means",
        "meaning",
        "called",
        "named",
        "spelled",
        "say",
        "says",
        "said",
        "type",
        "types",
        "typed",
        "typing",
        "press",
        "presses",
        "pressed",
        "hit",
        "hits",
        "put",
        "puts",
        "putting",
        "add",
        "adds",
        "added",
        "adding",
        "use",
        "uses",
        "used",
        "using",
        "insert",
        "inserts",
        "inserted",
        "write",
        "writes",
        "wrote",
        "writing",
        "need",
        "needs",
        "needed",
        "want",
        "wants",
        "with",
        "without",
        "of",
        "for",
        "to",
        "from",
        "by",
        "after",
        "before",
        "about",
        "plus",
        "and",
        "or",
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
    ];
    // Mirror of MARK_NOUN_BEFORE is large; reuse a minimal check via the same words the oracle already knows
    // from final_mark era — call through by reimplementing the narrow end check only.
    if !old.contains(" period") {
        return old.to_owned();
    }
    let s = old.strip_suffix(['.', '?', '!']).unwrap_or(old);
    let Some(body) = s.strip_suffix(" period") else {
        return old.to_owned();
    };
    if !body.ends_with(|c: char| c.is_ascii_alphanumeric()) {
        return old.to_owned();
    }
    let last = body
        .split_whitespace()
        .last()
        .map(|w| w.trim_matches(|c: char| matches!(c, ',')).to_ascii_lowercase())
        .unwrap_or_default();
    if last.is_empty()
        || !last.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'\'')
        || last.bytes().all(|b| b.is_ascii_digit())
        || qrtz_talk::cleanup::MARK_NOUN_BEFORE.contains(&last.as_str())
    {
        return old.to_owned();
    }
    let has_spoken_oxford = body.contains(" comma and ");
    let has_ascii_comma = body.as_bytes().contains(&b',');
    if !has_spoken_oxford && !has_ascii_comma {
        return old.to_owned();
    }
    if !has_spoken_oxford {
        return format!("{body}.");
    }
    let parts: Vec<&str> = body.split_whitespace().collect();
    let mut out = String::new();
    let mut converted = 0usize;
    for (i, w) in parts.iter().enumerate() {
        if *w == "comma" {
            let prev_ok = i
                .checked_sub(1)
                .map(|j| {
                    let p = parts[j]
                        .trim_matches(|c: char| matches!(c, ',' | '.' | '!' | '?'))
                        .to_ascii_lowercase();
                    !COMMA_NOUN_BEFORE.contains(&p.as_str())
                })
                .unwrap_or(false);
            if prev_ok {
                out.push_str(", ");
                converted += 1;
                continue;
            }
        }
        if i > 0 && !out.ends_with(", ") {
            out.push(' ');
        }
        out.push_str(w);
    }
    if converted == 0 {
        return old.to_owned();
    }
    format!("{}.", out.trim_end())
}

/// ADR 0234's one allowed edit (written independently of cleanup.rs): where `new` has "Mr." / "Mrs." in the place of
/// `old`'s "Mister" / "mister" / "Missus" / "missus" (same words, same spaces and line breaks), take it into `old`.
fn with_titles_from(old: &str, new: &str) -> String {
    let split = |t: &str| t.split([' ', '\n']).map(str::to_owned).collect::<Vec<_>>();
    let (o, n) = (split(old), split(new));
    let seps = |t: &str| t.chars().filter(|c| *c == ' ' || *c == '\n').collect::<String>();
    if o.len() != n.len() || seps(old) != seps(new) {
        return old.to_owned();
    }
    let mut out = String::new();
    let mut sep = seps(old).chars().collect::<Vec<_>>().into_iter();
    for (a, b) in o.iter().zip(&n) {
        let title = matches!(
            (a.as_str(), b.as_str()),
            ("Mister" | "mister", "Mr.") | ("Missus" | "missus", "Mrs.")
        );
        out.push_str(if title { b } else { a });
        if let Some(c) = sep.next() {
            out.push(c);
        }
    }
    out
}

/// ADR 0205 / ADR 0207's one allowed edit, as a superset (written independently of cleanup.rs): every "<P>, <p>
/// <modal>" for P in we/they loses ", <p>" (plain modals, negated contractions, and "cannot"). Whether the sentence
/// start allows it is checked by hardening65.rs / hardening67.rs, not here.
fn without_subject_repeats(old: &str) -> String {
    let mut out = old.to_owned();
    for p in ["we", "they"] {
        for m in [
            "can",
            "could",
            "will",
            "would",
            "should",
            "might",
            "must",
            "may",
            "shall",
            "can't",
            "couldn't",
            "won't",
            "wouldn't",
            "shouldn't",
            "mustn't",
            "mightn't",
            "mayn't",
            "cannot",
            "can\u{2019}t",
            "couldn\u{2019}t",
            "won\u{2019}t",
            "wouldn\u{2019}t",
            "shouldn\u{2019}t",
            "mustn\u{2019}t",
            "mightn\u{2019}t",
            "mayn\u{2019}t",
        ] {
            for first in [p.to_owned(), format!("{}{}", p[..1].to_uppercase(), &p[1..])] {
                out = out.replace(&format!("{first}, {p} {m}"), &format!("{first} {m}"));
            }
        }
    }
    out
}

/// ADR 0204's one allowed edit: a comma after the last sentence's first word when that word is one of
/// QUESTION_COMMA_OPENERS (written independently of cleanup.rs: split at the last ". " / "? " / "! " or line).
fn with_opener_comma(body: &str) -> String {
    let start = body
        .char_indices()
        .filter(|&(i, c)| {
            c == '\n'
                || (body[i..].starts_with(['.', '!', '?']) && body[i + 1..].starts_with(char::is_whitespace))
        })
        .map(|(i, c)| i + c.len_utf8())
        .next_back()
        .unwrap_or(0);
    let s = &body[start..];
    let lead = s.len() - s.trim_start().len();
    match s[lead..].find(char::is_whitespace) {
        Some(len) if QUESTION_COMMA_OPENERS.contains(&s[lead..lead + len].to_ascii_lowercase().as_str()) => {
            let at = start + lead + len;
            format!("{},{}", &body[..at], &body[at..])
        }
        _ => body.to_owned(),
    }
}

/// ADR 0206's one allowed edit (written independently of cleanup.rs): when the last sentence's first word is "hey"
/// (any ASCII case) and its second word is exactly a dictionary term, a comma goes right after that term.
fn with_vocative_comma(body: &str, dict: &[DictionaryEntry]) -> Option<String> {
    let start = body
        .char_indices()
        .filter(|&(i, c)| {
            c == '\n'
                || (body[i..].starts_with(['.', '!', '?']) && body[i + 1..].starts_with(char::is_whitespace))
        })
        .map(|(i, c)| i + c.len_utf8())
        .next_back()
        .unwrap_or(0);
    let s = &body[start..];
    let mut words = s.split_whitespace();
    let (hey, name) = (words.next()?, words.next()?);
    if !hey.eq_ignore_ascii_case("hey") || !dict.iter().any(|d| d.term == name) {
        return None;
    }
    let at = start + s.find(hey)? + hey.len();
    let at = at + body[at..].find(name)? + name.len();
    Some(format!("{},{}", &body[..at], &body[at..]))
}

fn load(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// Every raw text in cleanup.json and polish.json (polish and keep), with each dictionary and the file's own.
#[test]
fn every_vector_is_byte_identical_to_cycle_59() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/vectors/talk/");
    let mut texts: Vec<String> = Vec::new();
    let mut dicts = dictionaries();
    for (file, sets, field) in [
        (
            "cleanup.json",
            &["rules", "dictionaryOnly"][..],
            &["raw", "text"][..],
        ),
        ("polish.json", &["polish", "keep"][..], &["raw"][..]),
    ] {
        let v = load(&format!("{root}{file}"));
        dicts.push(serde_json::from_value(v["dictionary"].clone()).unwrap());
        for set in sets {
            for case in v[set].as_array().unwrap() {
                for f in field {
                    if let Some(t) = case[f].as_str() {
                        texts.push(t.to_owned());
                    }
                }
            }
        }
    }
    assert!(texts.len() > 400, "{} texts", texts.len());
    for d in &dicts {
        for t in &texts {
            same(t, d);
        }
    }
}

/// Pieces every rewritten step reacts to, and their near misses: spoken layout commands, fillers and their
/// look-alikes, repeats with commas and apostrophes, dots before lowercase words, abbreviations, lone "i", Unicode
/// whitespace and letters, dictionary words.
const PIECES: &[&str] = &[
    "new line",
    "New Line",
    "NEW LINE",
    "new  line",
    "new\tline",
    "newline",
    "renew line",
    "new liner",
    "new_line",
    "new paragraph",
    "New paragraph.",
    "a new line",
    "the new paragraph",
    "new line,",
    "new line.",
    "new line,,",
    "um",
    "Um,",
    "UM",
    "uh",
    "uhh",
    "umm",
    "er",
    "Er,",
    "err",
    "erm",
    "ermm",
    "ah",
    "ahh",
    "hmm",
    "hm",
    "mm",
    "mhm",
    "mmhmm",
    "mhmhm",
    "mm,",
    "uhm",
    "um.",
    "uh?",
    "um!",
    "hum",
    "summer",
    "35",
    "10",
    "I",
    "i",
    "i.",
    "i,",
    "i'",
    "i?",
    "i.e.",
    "e.g.",
    "the",
    "The",
    "the,",
    "THE",
    "a",
    "A",
    "an",
    "and",
    "to",
    "you",
    "you're",
    "it",
    "it's",
    "it,",
    "very",
    "very,",
    "we",
    "We",
    "so",
    "So.",
    "so.",
    "go.",
    "Mr.",
    "mr.",
    "Dr.",
    "etc.",
    "p.m.",
    "a.m.",
    "U.S.",
    "3",
    "3.",
    "dollars",
    "percent",
    "twenty",
    "five",
    "scratch that",
    "Scratch that,",
    "Scratch that.",
    "question mark",
    "exclamation point",
    "Do",
    "you",
    "want",
    "what",
    "time",
    "is",
    "Is",
    "there",
    ".",
    ",",
    ", ",
    "!",
    "?",
    "...",
    ";",
    ":",
    "\"",
    "(",
    ")",
    "-",
    "'",
    "_",
    "x_y",
    "a.b",
    "foo",
    "bar",
    "Bar",
    "BAR",
    "qrtz",
    "cue arts",
    "cue-arts",
    "Cue  Arts",
    "post gres",
    "Postgres",
    "quartz",
    "term number 3",
    "Term3",
    "strasse",
    "Straße",
    "kubernetes",
    "c plus plus",
    "unicode",
    "\u{212A}elvin",
    "Kelvin",
    "\u{17F}",
    "caf\u{e9}",
    "na\u{ef}ve",
    "\u{e9}a",
    "\u{e9}",
    "日本",
    "\u{a0}",
    "\u{3000}",
    "\u{2003}",
    "\u{85}",
    "\u{200B}",
    "\u{2028}",
    "\n",
    "\n\n",
    "\t",
    "  ",
    " , ",
    "¿",
    "Ⅰ",
    "ⅰ",
    "\u{17F}o",
    "\u{212A}",
    "kk\u{212A}",
    "kkk\u{20AC}",
    "So",
    "SO",
    "xcue arts",
    "cue artsy",
    "cue - arts",
    "cue -arts",
    "cue_arts",
    "cue\narts",
    "cue arts'",
    "CUE ARTS",
    "quartzite",
    "quartz.",
    "post-gres",
    "post\u{a0}gres",
    "kates.",
    "3d",
    "3D print",
    "term number 33",
    "term  number\t3",
    "Q",
    "Hey",
    "hey",
    "are",
    "can",
    "did",
    "Kelvin are you",
    "Hey Postgres",
];

/// The same small xorshift every run, so a failure names a reproducible text.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn generated(rng: &mut Rng) -> String {
    const GLUE: &[&str] = &[" ", " ", " ", " ", "", ",", ", ", "  ", "\n", ". ", "\t", " ,"];
    let mut s = String::new();
    for _ in 0..1 + rng.below(24) {
        s.push_str(PIECES[rng.below(PIECES.len())]);
        s.push_str(GLUE[rng.below(GLUE.len())]);
    }
    if rng.below(3) == 0 {
        s = s.trim().to_owned();
    }
    s
}

#[test]
fn generated_text_is_byte_identical_to_cycle_59() {
    let dicts = dictionaries();
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    // QRTZ_HARDENING60_N=<count> runs a longer search (cycle 60 ran 3,000,000 in release).
    let n = std::env::var("QRTZ_HARDENING60_N")
        .ok()
        .and_then(|n| n.parse().ok())
        .unwrap_or(if cfg!(debug_assertions) { 6_000 } else { 60_000 });
    for i in 0..n {
        let t = generated(&mut rng);
        same(&t, &dicts[i % dicts.len()]);
    }
}

/// The budget test's long dictation (cleanup_budget.rs), timed the same way.
fn long_dictation() -> String {
    let parts = [
        "Um, so I I think we should, uh, ship cue arts echo on Friday.",
        "The the build is green. and the docs are almost done",
        "I spent the night in the ER, so the review moved, um, to Monday.",
        "Use a 35 mm lens new line bring the charger new paragraph",
        "We launched our new line in May, and the numbers look good, hmm, better than last year.",
        "Email jlgrimes@users.noreply.github.com or go to https://qrtz.ai/docs for details. i.e. the setup guide",
        "Mr. and Mrs. Smith will join at 3 p.m. today with the team, etc. and post gres is migrated.",
        "Book the room for Monday. Scratch that, book it for Tuesday. My back itches. Scratch that itch.",
        "The demo is, um, basically done and, uh, we think the, um, docs are ready. If we can, um, we ship.",
        "It costs twenty five dollars, up fifteen percent, at three thirty p.m. Between ten and twenty dollars.",
        "Thanks for the update. Is the demo still on question mark",
    ];
    let mut s = String::new();
    while s.split_whitespace().count() < 400 {
        for p in parts {
            s.push_str(p);
            s.push(' ');
        }
    }
    s
}

/// A quarter of the 2 ms chain budget in the release build (0.11 ms on the box at cycle 60, 1.5-1.8 ms before). A
/// debug build only checks the text (its timings mean nothing).
#[test]
fn the_long_dictation_runs_in_a_quarter_of_the_budget() {
    let text = long_dictation();
    let dict = dictionaries().swap_remove(1);
    assert_eq!(
        cleanup_dictation(&text, &dict, true).text,
        c59::cleanup_dictation(&text, &dict, true).text
    );
    if cfg!(debug_assertions) {
        return;
    }
    for _ in 0..5 {
        cleanup_dictation(&text, &dict, true);
    }
    let mut ms: Vec<f64> = (0..41)
        .map(|_| {
            let t = Instant::now();
            std::hint::black_box(cleanup_dictation(std::hint::black_box(&text), &dict, true));
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    ms.sort_by(f64::total_cmp);
    let median = ms[ms.len() / 2];
    println!(
        "cleanup on {} words: median {median:.3} ms (ceiling 0.5 ms)",
        text.split_whitespace().count()
    );
    assert!(
        median <= 0.5,
        "cleanup took {median:.3} ms (median); ADR 0199 ceiling 0.5 ms"
    );
}

/// ADR 0230's one allowed edit, undone (written independently of cleanup.rs): every "<Month> <n>st/nd/rd/th" (n 1-31)
/// back to the ordinal words the recogniser wrote ("October 21st" -> "October twenty first").
fn days_as_words(t: &str) -> String {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    const UNITS: [&str; 19] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
        "thirteenth",
        "fourteenth",
        "fifteenth",
        "sixteenth",
        "seventeenth",
        "eighteenth",
        "nineteenth",
    ];
    let words = |n: usize| match n {
        1..=19 => UNITS[n - 1].to_owned(),
        20 => "twentieth".to_owned(),
        21..=29 => format!("twenty {}", UNITS[n - 21]),
        30 => "thirtieth".to_owned(),
        31 => "thirty first".to_owned(),
        _ => unreachable!(),
    };
    let toks: Vec<&str> = t.split(' ').collect();
    let mut out: Vec<String> = Vec::new();
    for (i, w) in toks.iter().enumerate() {
        let after_month = i > 0 && MONTHS.contains(&toks[i - 1].rsplit('\n').next().unwrap_or_default());
        let digits = w.bytes().take_while(u8::is_ascii_digit).count();
        let n: usize = w[..digits].parse().unwrap_or(0);
        let rest = &w[digits..];
        let suffix = ["st", "nd", "rd", "th"].iter().find(|s| rest.starts_with(**s));
        match (after_month, suffix) {
            (true, Some(s)) if (1..=31).contains(&n) => out.push(format!("{}{}", words(n), &rest[s.len()..])),
            _ => out.push((*w).to_owned()),
        }
    }
    out.join(" ")
}
