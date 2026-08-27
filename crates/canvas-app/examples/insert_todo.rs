fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = canvas_storage::Store::default_path();
    let store = canvas_storage::Store::open(&dir.join("canvas_local.db"))?;

    if std::env::args().any(|a| a == "--remove") {
        let mut removed = 0;
        for row in store.list_widgets()? {
            if row.widget_type == canvas_storage::KIND_TODO {
                store.delete_widget(&row.id)?;
                removed += 1;
            }
        }
        println!("removed {removed} todo rows");
        return Ok(());
    }

    let (row, _) = store.create_widget(canvas_storage::KIND_TODO)?;
    store.set_title(&row.id, "SEEDED TODO")?;

    let doc = canvas_doc::TodoListDoc::new();
    let (first, _) = doc.add_item("walk the verification queue");
    let (second, _) = doc.add_item("answer open question 9");
    let (third, _) = doc.add_item("look at the gallery");
    doc.add_item("no priority set on this one");
    doc.set_done(&first, true);
    doc.set_priority(&first, 1);
    doc.set_priority(&second, 3);
    doc.set_priority(&third, 2);
    store.save_document(&row.id, &doc.snapshot(), &doc.state_vector())?;

    println!(
        "inserted {} type={} with {} items ({:?} open, {:?} done)",
        row.id,
        row.widget_type,
        doc.items().len(),
        doc.counts().0,
        doc.counts().1
    );
    Ok(())
}
