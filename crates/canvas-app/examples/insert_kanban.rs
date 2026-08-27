fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = canvas_storage::Store::default_path();
    let store = canvas_storage::Store::open(&dir.join("canvas_local.db"))?;

    if std::env::args().any(|a| a == "--remove") {
        let mut removed = 0;
        for row in store.list_widgets()? {
            if row.widget_type == canvas_storage::KIND_KANBAN {
                store.delete_widget(&row.id)?;
                removed += 1;
            }
        }
        println!("removed {removed} kanban rows");
        return Ok(());
    }

    let (row, _) = store.create_widget(canvas_storage::KIND_KANBAN)?;
    store.set_title(&row.id, "SEEDED BOARD")?;

    let doc = canvas_doc::KanbanDoc::seeded();
    let columns = doc.columns();
    let (todo, doing, done) = (&columns[0].id, &columns[1].id, &columns[2].id);

    let due = canvas_storage::now_millis() + 3 * 86_400_000;

    let (bare, _) = doc.add_card(todo, "Nothing set on this one");
    let (assigned, _) = doc.add_card(todo, "Unlit dot value");
    let (dated, _) = doc.add_card(todo, "Re-shoot the drag state");
    let (both, _) = doc.add_card(doing, "Square-cap icon set");
    let (shipped, _) = doc.add_card(done, "32px header, all types");

    doc.set_assignee(&assigned, "mira");
    doc.set_priority(&assigned, 2);
    doc.set_due(&dated, due);
    doc.set_assignee(&both, "jonas");
    doc.set_due(&both, due);
    doc.set_priority(&both, 3);
    doc.set_priority(&shipped, 1);

    store.save_document(&row.id, &doc.snapshot(), &doc.state_vector())?;

    println!(
        "inserted {} type={} with {} cards ({} bare)",
        row.id,
        row.widget_type,
        doc.cards().len(),
        bare
    );
    Ok(())
}
