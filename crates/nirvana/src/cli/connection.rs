use chrono::{Local, TimeZone};
use console::{Style, Term};
use dialoguer::{Select, theme::ColorfulTheme};
use nirvana_core::api::{Connection, NirvanaApi};
use std::cmp::max;

enum Column {
    Unlimited(&'static str),
    Limited(&'static str, usize),
    Static(&'static str, usize),
}

impl Column {
    fn header(&self) -> &'static str {
        match self {
            Column::Unlimited(h) | Column::Limited(h, _) | Column::Static(h, _) => h,
        }
    }

    fn initial_width(&self) -> usize {
        match self {
            Column::Unlimited(h) => h.len(),
            Column::Limited(h, _) => h.len(),
            Column::Static(h, n) => max(h.len(), *n),
        }
    }

    fn fit(&self, current: usize, cell: &str) -> usize {
        match self {
            Column::Unlimited(_) => max(current, cell.len()),
            Column::Limited(_, cap) => max(current, cell.len().min(*cap)),
            Column::Static(_, _) => current,
        }
    }

    fn render(&self, cell: &str) -> String {
        match self {
            Column::Limited(_, cap) if cell.len() > *cap => {
                let mut s = cell[..cap.saturating_sub(1)].to_string();
                s.push('…');
                s
            }
            Column::Static(_, n) if cell.len() > *n => cell[..*n].to_string(),
            _ => cell.to_string(),
        }
    }
}

const COLUMNS: &[Column] = &[
    Column::Unlimited("ID"),
    Column::Static("ACTIVE", "ACTIVE".len()),
    Column::Limited("NAME", 15),
    Column::Unlimited("KIND"),
    Column::Limited("HOST", 25),
    Column::Limited("IDENTITY", 25),
    Column::Static("UPDATED", "2026-05-17".len()),
];

pub(crate) fn list() -> anyhow::Result<()> {
    let api = NirvanaApi::new()?;
    let connections = api.list_connections()?;

    let term = Term::stdout();
    if connections.is_empty() {
        term.write_line("No connections found.")?;
        term.write_line("Add one with: nirvana connection add")?;
        return Ok(());
    }

    let rows: Vec<[String; COLUMNS.len()]> = connections
        .iter()
        .map(|c| {
            let updated = Local
                .timestamp_opt(c.updated_at, 0)
                .single()
                .unwrap()
                .date_naive()
                .to_string();
            let active = if is_active(c, &api) { "*" } else { "" };
            [
                c.id.to_string(),
                active.to_string(),
                c.name.clone(),
                c.kind.clone(),
                c.base_url.clone(),
                c.identity.clone(),
                updated,
            ]
        })
        .collect();

    let mut widths: Vec<usize> = COLUMNS.iter().map(|c| c.initial_width()).collect();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = COLUMNS[i].fit(widths[i], cell);
        }
    }

    let bold = Style::new().bold();
    let header: Vec<&str> = COLUMNS.iter().map(|c| c.header()).collect();
    print_row(&term, &header, &widths, Some(&bold))?;

    for row in &rows {
        let cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, cell)| COLUMNS[i].render(cell))
            .collect();
        let cell_refs: Vec<&str> = cells.iter().map(|s| s.as_str()).collect();
        print_row(&term, &cell_refs, &widths, None)?;
    }

    Ok(())
}

fn is_active(conn: &Connection, api: &NirvanaApi) -> bool {
    api.active_connection() == Some(conn.id)
}

fn print_row(
    term: &Term,
    cells: &[&str],
    widths: &[usize],
    style: Option<&Style>,
) -> anyhow::Result<()> {
    debug_assert_eq!(cells.len(), widths.len());
    debug_assert_eq!(cells.len(), COLUMNS.len());

    let parts: Vec<String> = cells
        .iter()
        .zip(widths)
        .enumerate()
        .map(|(i, (cell, w))| {
            let padded = match i {
                0 => format!("{:>w$}", cell, w = *w),
                1 => format!("{:^w$}", cell, w = *w),
                _ => format!("{:<w$}", cell, w = *w),
            };
            match style {
                Some(s) => s.apply_to(padded).to_string(),
                None => padded,
            }
        })
        .collect();

    term.write_line(&parts.join("  "))?;
    Ok(())
}

pub(crate) fn activate(query: Option<&str>) -> anyhow::Result<()> {
    let mut api = NirvanaApi::new()?;
    let connections = api.list_connections()?;

    if connections.is_empty() {
        let term = Term::stdout();
        term.write_line("No connections found.")?;
        term.write_line("Add one with: nirvana connection add")?;
        return Ok(());
    }

    let idx = match query {
        Some(q) => resolve_query(q, &connections)?,
        None => pick_interactive(&connections)?,
    };
    let chosen = &connections[idx];

    api.set_active_connection(chosen.id)?;
    let term = Term::stdout();
    term.write_line(&format!(
        "Active connection set to '{}' (id {})",
        chosen.name, chosen.id
    ))?;
    Ok(())
}

fn resolve_query(query: &str, connections: &[Connection]) -> anyhow::Result<usize> {
    if let Ok(id) = query.parse::<i64>() {
        if let Some(idx) = connections.iter().position(|c| c.id == id) {
            return Ok(idx);
        }
    }
    if let Some(idx) = connections.iter().position(|c| c.name == query) {
        return Ok(idx);
    }
    anyhow::bail!("Connection '{query}' not found")
}

fn pick_interactive(connections: &[Connection]) -> anyhow::Result<usize> {
    let dim = Style::new().dim();
    let name_width = connections.iter().map(|c| c.name.len()).max().unwrap_or(0);

    let items: Vec<String> = connections
        .iter()
        .map(|c| {
            format!(
                "{:name_w$}  {} {}",
                c.name,
                dim.apply_to(&c.identity),
                dim.apply_to(format!("({})", c.kind)),
                name_w = name_width,
            )
        })
        .collect();

    let theme = ColorfulTheme {
        active_item_prefix: Style::new().green().apply_to(String::from("❯")),
        ..ColorfulTheme::default()
    };

    let idx = Select::with_theme(&theme)
        .with_prompt("Select a connection")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match idx {
        Some(i) => Ok(i),
        None => std::process::exit(0),
    }
}
