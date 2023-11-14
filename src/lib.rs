pub struct TableRenderer;

impl TableRenderer {
    const CHAR_CELL_SEPARATOR: char = '│';
    const CHAR_LINE_SEPARATOR: char = '─';
    const CHAR_JOIN_INNER: char = '┼';
    const CHAR_CORNER_TOP_LEFT: char = '┌';
    const CHAR_CORNER_TOP_RIGHT: char = '┐';
    const CHAR_JOIN_LEFT_INNER: char = '├';
    const CHAR_JOIN_RIGHT_INNER: char = '┤';
    const CHAR_JOIN_TOP_INNER: char = '┬';
    const CHAR_JOIN_BOTTOM_INNER: char = '┴';
    const CHAR_CORNER_BOTTOM_LEFT: char = '└';
    const CHAR_CORNER_BOTTOM_RIGHT: char = '┘';

    /// Renders the given data as a string representing a table.
    ///
    /// The first row of `data` is treated as the header.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `Vec<String>` where each `Vec<String>` represents a row in the table.
    ///
    /// # Examples
    ///
    /// ```
    /// use minimal_table::TableRenderer;
    ///
    /// let data = vec![
    ///     vec!["Header 1".to_string(), "Header 2".to_string()],
    ///     vec!["Row1".to_string(), "Row1Col2".to_string()],
    /// ];
    /// println!("{}", TableRenderer::render(&data));
    /// ```
    pub fn render(data: &[Vec<String>]) -> String {
        if data.is_empty() {
            return String::new();
        }

        let (headers, rows) = data.split_first().unwrap();
        let column_widths = Self::calculate_column_widths(headers, rows);

        let header = Self::render_row(
            headers,
            &column_widths,
            Self::CHAR_JOIN_TOP_INNER,
            Self::CHAR_CORNER_TOP_LEFT,
            Self::CHAR_CORNER_TOP_RIGHT,
        );

        let rows: Vec<String> = rows
            .iter()
            .map(|row| {
                Self::render_row(
                    row,
                    &column_widths,
                    Self::CHAR_JOIN_INNER,
                    Self::CHAR_JOIN_LEFT_INNER,
                    Self::CHAR_JOIN_RIGHT_INNER,
                )
            })
            .collect();

        let footer = Self::render_separator(
            &column_widths,
            Self::CHAR_JOIN_BOTTOM_INNER,
            Self::CHAR_CORNER_BOTTOM_LEFT,
            Self::CHAR_CORNER_BOTTOM_RIGHT,
        );

        format!("{}{}{}", header, rows.join(""), footer)
    }

    fn calculate_column_widths(headers: &[String], rows: &[Vec<String>]) -> Vec<usize> {
        let mut column_widths: Vec<usize> = vec![];
        for (i, header) in headers.iter().enumerate() {
            let mut max_width = header.chars().count();
            for row in rows {
                if let Some(cell) = row.get(i) {
                    max_width = max_width.max(cell.chars().count());
                }
            }
            column_widths.push(max_width + 2); // +2 for padding
        }
        column_widths
    }

    fn render_row(
        row: &[String],
        column_widths: &[usize],
        join_inner: char,
        corner_left: char,
        corner_right: char,
    ) -> String {
        let cells: Vec<String> = column_widths
            .iter()
            .enumerate()
            .map(|(i, &width)| {
                let empty_string = "".to_string();
                let cell_content = row.get(i).unwrap_or(&empty_string);
                let padded_content = format!(" {} ", cell_content);
                format!("{:width$}", padded_content, width = width)
            })
            .collect();

        let line = format!(
            "{}{}{}\n",
            Self::CHAR_CELL_SEPARATOR,
            cells.join(&Self::CHAR_CELL_SEPARATOR.to_string()),
            Self::CHAR_CELL_SEPARATOR
        );

        let separator =
            Self::render_separator(column_widths, join_inner, corner_left, corner_right);

        format!("{}{}", separator, line)
    }

    fn render_separator(
        column_widths: &[usize],
        join_inner: char,
        corner_left: char,
        corner_right: char,
    ) -> String {
        let separators: Vec<String> = column_widths
            .iter()
            .map(|&width| Self::CHAR_LINE_SEPARATOR.to_string().repeat(width))
            .collect();

        format!(
            "{}{}{}\n",
            corner_left,
            separators.join(&join_inner.to_string()),
            corner_right
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_rendering() {
        let data = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Row1".to_string(), "Row1Col2".to_string()],
        ];
        let rendered_table = TableRenderer::render(&data);
        let expected_table = "\
┌──────────┬──────────┐\n\
│ Header 1 │ Header 2 │\n\
├──────────┼──────────┤\n\
│ Row1     │ Row1Col2 │\n\
└──────────┴──────────┘\n";

        assert_eq!(rendered_table, expected_table);
    }
}
