//! CLI output formatting utilities.
//!
//! Provides consistent table and detail formatting for all subcommands.
//! Follows kubectl table pattern: CAPS header, space-padded columns.

/// Renders a table with CAPS header row and space-padded columns.
///
/// Each row is a `Vec<String>` with values for each column.
/// Columns are padded to the widest value (including header).
pub fn render_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let col_count = headers.len();

    // Calculate max width per column (header width as minimum)
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, val) in row.iter().enumerate() {
            if i < col_count {
                widths[i] = widths[i].max(val.len());
            }
        }
    }

    let mut lines = Vec::new();

    // Header row (CAPS)
    let header_line: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h.to_uppercase(), width = widths[i]))
        .collect();
    lines.push(header_line.join("  "));

    // Data rows
    for row in rows {
        let data_line: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let w = widths.get(i).copied().unwrap_or(val.len());
                // Last column: don't pad (avoids trailing spaces)
                if i == col_count - 1 {
                    val.clone()
                } else {
                    format!("{val:<w$}")
                }
            })
            .collect();
        lines.push(data_line.join("  "));
    }

    lines.join("\n")
}

/// Renders a detail view with aligned key-value pairs.
///
/// Keys are padded to the widest key length, followed by value.
/// Blank lines can be inserted by passing an empty key+value tuple.
pub fn render_detail(pairs: &[(&str, String)]) -> String {
    let max_key = pairs
        .iter()
        .filter(|(k, _)| !k.is_empty())
        .map(|(k, _)| k.len())
        .max()
        .unwrap_or(0);

    let mut lines = Vec::new();
    for (key, value) in pairs {
        if key.is_empty() {
            lines.push(String::new());
        } else {
            lines.push(format!("{key:<max_key$}  {value}"));
        }
    }

    lines.join("\n")
}

/// Renders a count heading like "Plugins (13)" or "Plugins oficiais (13)".
pub fn render_count_heading(label: &str, count: usize) -> String {
    format!("{label} ({count})")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn render_table_basic() {
        let headers = &["NAME", "KIND", "STATUS"];
        let rows = vec![
            vec![
                "official.claude".to_owned(),
                "agent".to_owned(),
                "enabled".to_owned(),
            ],
            vec![
                "official.basic".to_owned(),
                "template".to_owned(),
                "enabled".to_owned(),
            ],
        ];

        let output = render_table(headers, &rows);
        assert!(output.contains("NAME"));
        assert!(output.contains("official.claude"));
        // Columns should be aligned
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 rows
    }

    #[test]
    fn render_table_empty() {
        let output = render_table(&["A", "B"], &[]);
        assert!(output.is_empty());
    }

    #[test]
    fn render_detail_basic() {
        let pairs = vec![
            ("Plugin:", "official.claude".to_owned()),
            ("Kind:", "agent_runtime".to_owned()),
            ("", String::new()),
            ("Status:", "enabled".to_owned()),
        ];
        let output = render_detail(&pairs);
        assert!(output.contains("Plugin:"));
        assert!(output.contains("official.claude"));
        // Should have a blank line separator
        assert!(output.contains("\n\n"));
    }

    #[test]
    fn render_count_heading_basic() {
        let output = render_count_heading("Plugins", 13);
        assert_eq!(output, "Plugins (13)");
    }
}
