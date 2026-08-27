use canvas_storage::{KIND_CALENDAR, KIND_KANBAN, KIND_SPREADSHEET, KIND_STICKY_NOTE, KIND_TODO};

pub struct WidgetType {
    pub kind: &'static str,
    pub name: &'static str,
    pub label: &'static str,
    pub shortcut: &'static str,
}

pub const BUILT: &[WidgetType] = &[
    WidgetType {
        kind: KIND_STICKY_NOTE,
        name: "note",
        label: "NEW NOTE",
        shortcut: "CTRL+N",
    },
    WidgetType {
        kind: KIND_TODO,
        name: "todo",
        label: "NEW TODO",
        shortcut: "CTRL+T",
    },
    WidgetType {
        kind: KIND_CALENDAR,
        name: "calendar",
        label: "NEW CALENDAR",
        shortcut: "",
    },
    WidgetType {
        kind: KIND_KANBAN,
        name: "board",
        label: "NEW BOARD",
        shortcut: "",
    },
    WidgetType {
        kind: KIND_SPREADSHEET,
        name: "sheet",
        label: "NEW SHEET",
        shortcut: "",
    },
];

pub const NEW_PREFIX: &str = "new:";

pub fn action_id(kind: &str) -> String {
    format!("{NEW_PREFIX}{kind}")
}

pub fn kind_for_action(id: &str) -> Option<&'static str> {
    let kind = id.strip_prefix(NEW_PREFIX)?;
    BUILT.iter().find(|t| t.kind == kind).map(|t| t.kind)
}

pub fn palette_label(t: &WidgetType) -> String {
    format!("New {}", t.name)
}

pub fn resolve_kind(word: &str) -> Option<&'static str> {
    let word = word.trim().to_ascii_lowercase();
    BUILT
        .iter()
        .find(|t| t.kind == word || t.name == word)
        .map(|t| t.kind)
}

pub fn kind_words() -> String {
    let mut words: Vec<String> = Vec::new();
    for t in BUILT {
        if t.kind == t.name {
            words.push(t.kind.to_string());
        } else {
            words.push(format!("{} ({})", t.kind, t.name));
        }
    }
    words.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::WidgetDoc;

    const ALL_KINDS: &[&str] = &[
        KIND_STICKY_NOTE,
        KIND_TODO,
        KIND_CALENDAR,
        KIND_KANBAN,
        KIND_SPREADSHEET,
    ];

    #[test]
    fn every_offered_type_has_a_document_behind_it() {
        for t in BUILT {
            assert!(
                WidgetDoc::load(t.kind, None).is_some(),
                "{} is offered but has no document type",
                t.kind
            );
        }
    }

    #[test]
    fn a_kind_that_gains_a_document_must_be_added_to_this_table() {
        for kind in ALL_KINDS {
            let has_doc = WidgetDoc::load(kind, None).is_some();
            let offered = BUILT.iter().any(|t| t.kind == *kind);
            assert_eq!(
                has_doc, offered,
                "{kind} has a document but is not offered, or the other way round"
            );
        }
    }

    #[test]
    fn the_removed_document_kind_is_inert_rather_than_a_panic() {
        assert!(WidgetDoc::load("document", None).is_none());
        assert_eq!(resolve_kind("document"), None);
        assert_eq!(kind_for_action("new:document"), None);
    }

    #[test]
    fn a_kind_resolves_from_its_storage_key_or_its_spoken_name() {
        assert_eq!(resolve_kind("kanban"), Some(KIND_KANBAN));
        assert_eq!(resolve_kind("board"), Some(KIND_KANBAN));
        assert_eq!(resolve_kind("  BOARD "), Some(KIND_KANBAN));
        assert_eq!(resolve_kind("sticky_note"), Some(KIND_STICKY_NOTE));
        assert_eq!(resolve_kind("note"), Some(KIND_STICKY_NOTE));
        assert_eq!(resolve_kind("whiteboard"), None);
        assert_eq!(resolve_kind(""), None);
    }

    #[test]
    fn every_built_type_is_reachable_by_both_of_its_words() {
        for t in BUILT {
            assert_eq!(resolve_kind(t.kind), Some(t.kind));
            assert_eq!(resolve_kind(t.name), Some(t.kind));
        }
    }

    #[test]
    fn the_help_line_names_every_built_type() {
        let words = kind_words();
        for t in BUILT {
            assert!(words.contains(t.kind), "{} missing from {words}", t.kind);
        }
    }

    #[test]
    fn no_two_types_share_a_kind() {
        for (i, a) in BUILT.iter().enumerate() {
            for b in &BUILT[i + 1..] {
                assert_ne!(a.kind, b.kind);
            }
        }
    }

    #[test]
    fn an_action_id_round_trips_to_its_kind() {
        for t in BUILT {
            assert_eq!(kind_for_action(&action_id(t.kind)), Some(t.kind));
        }
    }

    #[test]
    fn an_action_id_is_not_a_widget_id() {
        assert_eq!(
            kind_for_action("b6ff3785-0d18-434e-97c5-c9e5f4e21b3f"),
            None
        );
        assert_eq!(kind_for_action("settings"), None);
        assert_eq!(kind_for_action("new:whiteboard"), None);
    }

    #[test]
    fn a_palette_label_reads_as_a_sentence() {
        let labels: Vec<String> = BUILT.iter().map(palette_label).collect();
        assert_eq!(
            labels,
            [
                "New note",
                "New todo",
                "New calendar",
                "New board",
                "New sheet"
            ]
        );
    }

    #[test]
    fn a_hover_label_is_uppercase_and_names_its_type() {
        for t in BUILT {
            assert_eq!(t.label, t.label.to_uppercase());
            assert!(t.label.starts_with("NEW "));
            assert!(t.label.ends_with(&t.name.to_uppercase()));
        }
    }
}
