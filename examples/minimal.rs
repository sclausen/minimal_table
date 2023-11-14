use minimal_table::TableRenderer;

fn main() {
    let data = vec![
        vec![
            "Header 1".to_string(),
            "Header 2".to_string(),
            "Header 3".to_string(),
        ],
        vec![
            "Foobar".to_string(),
            "Longer text to test".to_string(),
            "Gnarpeldarp".to_string(),
        ],
        vec!["Schnorpeldorp".to_string(), "Gnobbelblob".to_string()],
    ];
    let table = TableRenderer::render(&data);

    println!("{}", table);
}
