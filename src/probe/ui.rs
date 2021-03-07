use crate::config::Probe;
use crate::probe::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Row, Table, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        // 1 => draw_second_tab(f, app, chunks[1]),
        // 2 => draw_third_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Max(10),
                Constraint::Max(10),
                Constraint::Max(10),
            ]
            .as_ref(),
        )
        .split(area);
    draw_probe(f, &app.probes[0], chunks[0]);
}

fn draw_probe<B>(f: &mut Frame<B>, probe: &Probe, area: Rect)
where
    B: Backend,
{
    // let chunks = Layout::default()
    //     .constraints(
    //         [
    //             Constraint::Length(2),
    //             Constraint::Length(3),
    //             Constraint::Length(1),
    //         ]
    //         .as_ref(),
    //     )
    //     .margin(1)
    //     .split(area);
    let block = Block::default().borders(Borders::ALL).title("Probe 1");
    f.render_widget(block, area);

    let style = Style::default().fg(Color::White);
    let mut rows = match &probe.filters {
        Some(filters) => filters
            .iter()
            .map(|p| Row::new(vec![p.name.to_string(), p.count.to_string()]).style(style))
            .collect(),
        None => Vec::new(),
    };
    // let mut rows: Vec<Row> = probe
    //     .filters
    //     .iter()
    //     .map(|p| Row::new(vec![p.name.to_string(), p.count.to_string()]).style(style))
    //     .collect();
    rows.insert(
        0,
        Row::new(vec![String::from("All"), probe.count.to_string()]).style(style),
    );
    let table = Table::new(rows)
        .header(
            Row::new(vec!["Type", "Count"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .title(probe.name.clone())
                .borders(Borders::ALL),
        )
        .widths(&[Constraint::Length(10), Constraint::Length(6)]);
    f.render_widget(table, area);
}
