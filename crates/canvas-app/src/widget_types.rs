use canvas_storage::{KIND_CALENDAR, KIND_DOCUMENT, KIND_KANBAN, KIND_STICKY_NOTE, KIND_TODO};

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
        kind: KIND_DOCUMENT,
        name: "document",
        label: "NEW DOCUMENT",
        shortcut: "",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::WidgetDoc;

    const ALL_KINDS: &[&str] = &[
        KIND_STICKY_NOTE,
        KIND_TODO,
        KIND_DOCUMENT,
        KIND_CALENDAR,
        KIND_KANBAN,
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
        assert_eq!(kind_for_action("new:spreadsheet"), None);
    }

    #[test]
    fn a_palette_label_reads_as_a_sentence() {
        let labels: Vec<String> = BUILT.iter().map(palette_label).collect();
        assert_eq!(
            labels,
            [
                "New note",
                "New todo",
                "New document",
                "New calendar",
                "New board"
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
