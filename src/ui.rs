use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    text::{Line, Span},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
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

fn draw_tab_bar<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    Span::raw("This is the:"),
                    Span::raw("Header"),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("Tab bar").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn draw_footer<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    Span::raw("This is the:"),
                    Span::raw("Footer"),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("Footer").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn draw_body<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
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

fn draw_wf_list<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    Span::raw("This is the:"),
                    Span::raw("wf_list"),
                    Span::raw("list"),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("wf list").borders(Borders::ALL));

    frame.render_widget(content, area);
}

fn draw_wf_summary<B: Backend>(frame: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Line::from(vec![
                    Span::raw("This is the:"),
                    Span::raw("wf_summary"),
                    Span::raw("lorem ipsum"),
        ]),
    ];

    let content = Paragraph::new(text)
        .block(Block::default().title("wf summary").borders(Borders::ALL));

    frame.render_widget(content, area);
}
