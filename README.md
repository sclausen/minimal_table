# Minimal Table

A minimal table implementation that renders a Vec<Vec<String>> to a string table.

## Examples

```rust
use minimal_table::TableRenderer;

let data = vec![
    vec!["Header 1".to_string(), "Header 2".to_string()],
    vec!["Row1".to_string(), "Row1Col2".to_string()],
];
let table = TableRenderer::render(&data);
println!("{}", table);
```

## Contributing

Contributions are welcome but keep in mind, that this should be a minimal library. If you would've expected a full featured amazing table library, please consider using [Stanza](https://github.com/obsidiandynamics/stanza)

## License

[MIT](LICENSE)