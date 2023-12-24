use crate::probe::app::App;
use crate::probe::state::Probe;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Sparkline, Table, Tabs, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    if app.state.detail_view {
        draw_detail(f, app);
    } else {
        draw_list(f, app);
    }
}
pub fn draw_detail(f: &mut Frame, app: &mut App) {
    let text = app.selected_probe().messages();
    let p = Paragraph::new(text)
        .block(
            Block::default()
                .title(app.selected_probe().name)
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(p, f.size());
}

pub fn draw_list(f: &mut Frame, app: &mut App) {
    let num_probes = app.state.probes.len();
    let probes_per_tab = (f.size().height as usize - 3) / 5;
    app.tabs.recalculate_layout(num_probes, probes_per_tab);
    let titles: Vec<Line> = (0..app.tabs.num_tabs)
        .map(|t| {
            Line::from(Span::styled(
                format!("Page {}", t + 1),
                Style::default().fg(Color::White),
            ))
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Blue))
        .select(app.tabs.selected_tab);

    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());
    f.render_widget(tabs, chunks[0]);
    draw_tab(f, app, chunks[1]);

    let help_text = Line::raw(String::from(
        "j/k: up/down; enter: show/hide details; h/l: prev/next page; q: quit",
    ));
    let p = Paragraph::new(help_text)
        .block(Block::default().title("Keys").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    f.render_widget(p, chunks[2]);
}

fn draw_tab(f: &mut Frame, app: &App, area: Rect) {
    // create blocks for each probe
    let probes = app.probes_for_tab();
    let num_probes = probes.len();
    let constraints: Vec<Constraint> = (0..num_probes + 1)
        .map(|_c| Constraint::Length(5))
        .collect();
    let chunks = Layout::default().constraints(constraints).split(area);

    // for each probe, draw it in a chunk
    probes.iter().enumerate().for_each(|(i, p)| {
        let style = if i == app.tabs.selected_probe {
            Style::default().fg(Color::Blue)
        } else {
            Style::default().fg(Color::White)
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title(p.name.clone())
            .style(style);
        f.render_widget(block, chunks[i]);
        draw_probe(f, p, chunks[i]);
    });
}

fn draw_probe(f: &mut Frame, probe: &Probe, area: Rect) {
    // split the area in two: left for the table, right for the histogram
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(62)].as_ref())
        .margin(1)
        .split(area);

    let style = Style::default().fg(Color::White);

    let rows = vec![Row::new(vec![probe.filter.clone(), probe.count.to_string()]).style(style)];

    let widths = [Constraint::Length(8), Constraint::Length(6)];
    let table = Table::new(rows, widths).header(
        Row::new(vec!["Match", "Count"])
            .style(Style::default().fg(Color::White))
            .bottom_margin(1),
    );
    f.render_widget(table, chunks[0]);

    // fill the histogram
    let data = probe.histogram();
    let sparkline = Sparkline::default()
        .block(
            Block::default()
                .title("Histogram")
                .style(Style::default().fg(Color::White)),
        )
        .style(Style::default().fg(Color::Blue))
        .data(&data[..])
        .bar_set(ratatui::symbols::bar::THREE_LEVELS);
    f.render_widget(sparkline, chunks[1]);
}
