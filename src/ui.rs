use ratatui::{
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame
};

use crate::{App, InputMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    // レイアウト設定
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref() as &[_])
        .split(vertical_chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)] as &[_])
        .split(top_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref() as &[Constraint])
        .split(top_chunks[1]);

    // ボーダーのみ緑色に設定、内部の文字列はデフォルト
    let green_border = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Green));
    let default_style = Style::default(); // テキストはデフォルトスタイル

    // MainMenu描画 (Normalモード時に枠を緑に)
    let main_menu_block = if app.input_mode == InputMode::Normal {
        green_border.clone().title("Main Menu")
    } else {
        Block::default().borders(Borders::ALL).title("Main Menu")
    };

    // サブメニューとメインメニューの描画
    if app.in_sub_menu {
        let items: Vec<ListItem> = app
            .sub_items
            .iter()
            .map(|(item, selected)| {
                let prefix = if *selected { "[x] " } else { "[ ] " };
                ListItem::new(format!("{}{}", prefix, item)).style(default_style) // 文字列のスタイル
            })
            .collect();

        let sub_menu_widget = List::new(items)
            .block(main_menu_block)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        f.render_stateful_widget(sub_menu_widget, left_chunks[0], &mut app.sub_state);
    } else {
        let items: Vec<ListItem> = app
            .main_items
            .iter()
            .map(|i| ListItem::new(i.clone()).style(default_style)) // 文字列のスタイル
            .collect();

        let main_menu_widget = List::new(items)
            .block(main_menu_block)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        f.render_stateful_widget(main_menu_widget, left_chunks[0], &mut app.main_state);
    }

    // TextBox描画 (枠は通常のまま)
    let text_content: Vec<Line> = app
        .messages
        .iter()
        .map(|msg| Line::from(Span::raw(msg.clone())))
        .collect();
    let text_widget = Paragraph::new(Text::from(text_content))
        .block(Block::default().borders(Borders::ALL).title("Text Box"));
    f.render_widget(text_widget, left_chunks[1]);

    // Status描画 (枠は通常のまま)
    let status_content = ["Version: 1.0.0".to_string(),
        "Version2: 2.0.1".to_string(),
        "Path: /usr/local/bin".to_string(),
        "ServerInfo: Online".to_string(),
        "ServerMember: 42".to_string()];

    let status_items: Vec<ListItem> = status_content
        .iter()
        .map(|status| ListItem::new(status.clone()).style(default_style)) // 文字列のスタイル
        .collect();

    let status_widget = List::new(status_items)
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status_widget, right_chunks[0]);

    // Profile描画 (枠は通常のまま)
    let checked_items: Vec<ListItem> = app
        .sub_items
        .iter()
        .filter(|(_, selected)| *selected)
        .map(|(item, _)| ListItem::new(item.clone()).style(default_style)) // 文字列のスタイル
        .collect();

    let profile_widget = List::new(checked_items)
        .block(Block::default().borders(Borders::ALL).title("Profile"));
    f.render_widget(profile_widget, right_chunks[1]);

    // Console描画 (Editingモード時に枠を緑に)
    let console_block = if app.input_mode == InputMode::Editing {
        green_border.clone().title("Console")
    } else {
        Block::default().borders(Borders::ALL).title("Console")
    };

    let console_widget = Paragraph::new(app.console_input.clone()).style(default_style)
        .block(console_block);
    f.render_widget(console_widget, vertical_chunks[1]);

    // カーソル位置を表示 (Editingモード時のみ)
    if app.input_mode == InputMode::Editing {
        let cursor_position = Position::new(
            vertical_chunks[1].x + app.cursor_position as u16 + 1,  // Console領域にカーソルを設定
            vertical_chunks[1].y + 1,
        );
        f.set_cursor_position(cursor_position);
    }
}
