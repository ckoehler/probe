use crate::probe::app::App;
use crate::probe::state::ProbeState;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Sparkline, Table, Tabs, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if app.state.detail_view {
        draw_detail(f, app);
    } else {
        draw_list(f, app);
    }
}
pub fn draw_detail<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let text = app.state.selected_probe().messages();
    let p = Paragraph::new(text)
        .block(
            Block::default()
                .title(app.state.selected_probe().name)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(p, f.size());
}

pub fn draw_list<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let num_probes = app.state.probes.len() as u16;
    let probes_per_tab: u16 = (f.size().height - 3) / 5;
    let num_tabs = ((num_probes as f64 / probes_per_tab as f64).ceil()) as u16;
    let titles: Vec<Spans> = (0..num_tabs)
        .map(|t| {
            Spans::from(Span::styled(
                format!("Page {}", t + 1),
                Style::default().fg(Color::Green),
            ))
        })
        .collect();
    app.tabs.num = num_tabs as usize;
    app.tabs.probe_num = probes_per_tab as usize;
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    f.render_widget(tabs, chunks[0]);
    draw_tab(f, app, chunks[1]);
}

fn draw_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    // create blocks for each probe
    let probes = app.probes_for_tab();
    let num_probes = probes.len();
    let constraints: Vec<Constraint> = (0..num_probes + 1)
        .map(|_c| Constraint::Length(5))
        .collect();
    let chunks = Layout::default().constraints(constraints).split(area);

    // for each probe, draw it in a chunk
    probes.iter().enumerate().for_each(|(i, p)| {
        let block = Block::default().borders(Borders::ALL).title(p.name.clone());
        f.render_widget(block, chunks[i]);
        draw_probe(f, &p, i == app.state.selected_probe, chunks[i]);
    });
}

fn draw_probe<B>(f: &mut Frame<B>, probe: &ProbeState, selected: bool, area: Rect)
where
    B: Backend,
{
    // split the area in two: left for the table, right for the histogram
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(62)].as_ref())
        .margin(1)
        .split(area);

    let style = if selected {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::White)
    };

    let mut rows = Vec::new();
    rows.push(Row::new(vec![probe.filter.clone(), probe.count.to_string()]).style(style));
    let table = Table::new(rows)
        .header(
            Row::new(vec!["Match", "Count"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .widths(&[Constraint::Length(8), Constraint::Length(6)]);
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
