use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, List, ListItem, ListState},
    text::{Line, Text},
    Frame,
};

use std::collections::HashMap;

use crate::app::App;
use swf::workflow::Workflow;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let top_level = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Min(20),
                Constraint::Length(5),
            ].as_ref())
        .split(frame.size());

    draw_tab_bar(frame, app, top_level[0]);
    draw_body(frame, app, top_level[1]);
    draw_footer(frame, app, top_level[2]);
}

fn draw_tab_bar(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    "This is the:".into(),
                    "Header".into(),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("Tab bar").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn draw_footer(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    "This is the:".into(),
                    "Footer".into(),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("Footer").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn draw_body(frame: &mut Frame, app: &mut App, area: Rect) {
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(30),
                Constraint::Min(5),
            ].as_ref())
        .split(area);

    draw_wf_list(frame, app, body_layout[0]);
    draw_wf_summary(frame, app, body_layout[1]);
}

fn draw_wf_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let wf_names = app.workflows.iter()
                                .map(|x| ListItem::new(String::from(&x.name)))
                                .collect::<Vec<ListItem>>();

    let mut lstate = ListState::default();
    lstate.select(Some(app.selected_wf));

    let content = List::new(wf_names)
        .block(Block::default().title("wf list").borders(Borders::ALL))
        .highlight_style(
            Style::default()
            .bg(Color::LightBlue)
            .fg(Color::Black)
        );

    // frame.render_widget(content, area);
    frame.render_stateful_widget(content, area, &mut lstate);
}

fn draw_wf_summary(frame: &mut Frame, app: &mut App, area: Rect) {
    let text = Text::from(format_wf_summary(&app.workflows[app.selected_wf]));
    let content = Paragraph::new(text)
        .block(Block::default().title("wf summary").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn format_wf_summary(wf: &Workflow) -> Vec<Line> {
    let l_name = Line::from(vec![
        "Workflow Name: ".into(),
        wf.name.to_string().into(),
    ]);
    let l_steps = Line::from(vec![
        "Step Count: ".into(),
        wf.n_steps.to_string().into(),
    ]);

    let mut root_path = String::new();
    if let Some(p) =  &wf.root_path {
        root_path.push_str(p.to_str().unwrap_or(""));
    }

    let l_path = Line::from(vec![
        "Root Path: ".into(),
        root_path.into(),
    ]);

    let mut formatted_out = vec![l_name, l_steps, l_path];

    if let Some(opts) = &wf.default_sbatch_opts {
        formatted_out.push(Line::from("Default sbatch Options:"));

        let mut fmt_opts = format_sbatch_options(opts);
        formatted_out.append(&mut fmt_opts);
    }

    formatted_out
}

fn format_sbatch_options(opts: &HashMap<String, String>) -> Vec<Line> {
    opts.iter()
        .map(|(k, v)| Line::from(vec![
                                 "| ".into(), k.into(), ": ".into(), v.into()]))
        .collect()
}
