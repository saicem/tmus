use serde::Serialize;

/// Print a value as pretty-printed JSON to stdout.
pub fn print_json<T: Serialize>(value: &T) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).map_err(|e| e.to_string())?
    );
    Ok(())
}

/// Print an aligned text table. No-op when there are no rows.
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        return;
    }
    let mut widths: Vec<usize> = headers.iter().map(|h| h.chars().count()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if let Some(width) = widths.get_mut(i) {
                *width = (*width).max(cell.chars().count());
            }
        }
    }
    let format_row = |cells: &[String]| -> String {
        cells
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{:<width$}", cell, width = widths[i]))
            .collect::<Vec<_>>()
            .join("  ")
    };
    let header: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    println!("{}", format_row(&header));
    for row in rows {
        println!("{}", format_row(row));
    }
}

/// Format milliseconds as a human friendly duration, e.g. "1h 2m 3s".
pub fn format_duration(ms: i64) -> String {
    let secs = ms / 1000;
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    if hours > 0 {
        format!("{hours}h {minutes}m {seconds}s")
    } else if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
}

/// Format a unix millisecond timestamp as a local RFC-3339 string.
pub fn format_timestamp(ms: i64) -> String {
    chrono::DateTime::from_timestamp_millis(ms)
        .map(|dt| dt.with_timezone(&chrono::Local).to_rfc3339())
        .unwrap_or_else(|| ms.to_string())
}
