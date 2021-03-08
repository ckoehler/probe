use crate::probe::app::App;
use crate::probe::state::ProbeState;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Row, Sparkline, Table, Tabs},
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
    // create blocks for each probe
    let num_probes = app.state.probes.len();
    let constraints: Vec<Constraint> = (0..num_probes).map(|_c| Constraint::Min(7)).collect();
    let chunks = Layout::default().constraints(constraints).split(area);

    // for each probe, draw it in a chunk
    app.state.probes.iter().enumerate().for_each(|(i, p)| {
        let block = Block::default().borders(Borders::ALL).title(p.name.clone());
        f.render_widget(block, chunks[i]);
        draw_probe(f, &p, chunks[i]);
    });
}

fn draw_probe<B>(f: &mut Frame<B>, probe: &ProbeState, area: Rect)
where
    B: Backend,
{
    // let block = Block::default()
    //     .borders(Borders::ALL)
    //     .title(probe.name.clone());
    // f.render_widget(block, area);

    // split the area in two: left for the table, right for the histogram
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(62)].as_ref())
        .margin(1)
        .split(area);

    let style = Style::default().fg(Color::White);
    // Build all the rows from filters
    let mut rows: Vec<Row> = probe
        .filters
        .iter()
        .map(|p| Row::new(vec![p.name.to_string(), "0".to_string()]).style(style))
        .collect();

    // Add a row for all messages at the front
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
        .widths(&[Constraint::Length(10), Constraint::Length(6)]);
    f.render_widget(table, chunks[0]);

    // fill the histogram
    let data = probe.histogram();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Histogram")
                .style(Style::default().fg(Color::Yellow)),
        )
        .style(Style::default().fg(Color::Green))
        .data(&data[..])
        .bar_set(tui::symbols::bar::THREE_LEVELS);
    f.render_widget(sparkline, chunks[1]);
}
