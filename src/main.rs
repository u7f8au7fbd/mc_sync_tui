use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::ListState, Terminal
};
use std::io::stdout;
mod ui;

struct App {
    main_items: Vec<String>,
    sub_items: Vec<(String, bool)>,
    main_state: ListState,
    sub_state: ListState,
    selected_text: String,
    in_sub_menu: bool,
    input_mode: InputMode,
    console_input: String,
    cursor_position: usize, // カーソル位置
    messages: Vec<String>,  // 入力履歴用のメッセージリスト
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

impl App {
    fn new() -> App {
        let mut main_state = ListState::default();
        main_state.select(Some(0));
        App {
            main_items: vec![
                String::from("Item 1"),
                String::from("Item 2"),
                String::from("Item 3"),
                String::from("Item 4"),
                String::from("Menu"),
            ],
            sub_items: vec![
                (String::from("Sub Item 1"), false),
                (String::from("Sub Item 2"), false),
                (String::from("Sub Item 3"), false),
                (String::from("Sub Item 4"), false),
            ],
            main_state,
            sub_state: ListState::default(),
            selected_text: String::new(),
            in_sub_menu: false,
            input_mode: InputMode::Normal,
            console_input: String::new(),
            cursor_position: 0, // カーソル位置の初期化
            messages: Vec::new(),
        }
    }

    fn move_up(&mut self) {
        if self.in_sub_menu {
            let i = self.sub_state.selected().unwrap_or(0);
            if i > 0 {
                self.sub_state.select(Some(i - 1));
            }
        } else {
            let i = self.main_state.selected().unwrap_or(0);
            if i > 0 {
                self.main_state.select(Some(i - 1));
            }
        }
    }

    fn move_down(&mut self) {
        if self.in_sub_menu {
            let i = self.sub_state.selected().unwrap_or(0);
            if i < self.sub_items.len() - 1 {
                self.sub_state.select(Some(i + 1));
            }
        } else {
            let i = self.main_state.selected().unwrap_or(0);
            if i < self.main_items.len() - 1 {
                self.main_state.select(Some(i + 1));
            }
        }
    }

    fn select_item(&mut self) {
        if self.in_sub_menu {
            if let Some(i) = self.sub_state.selected() {
                self.sub_items[i].1 = !self.sub_items[i].1;
            }
        } else if let Some(i) = self.main_state.selected() {
            if self.main_items[i] == "Menu" {
                self.in_sub_menu = true;
                self.sub_state.select(Some(0));
            } else {
                self.selected_text = format!("Selected: {}", self.main_items[i]);
            }
        }
    }

    fn back_to_main_menu(&mut self) {
        if self.in_sub_menu {
            self.in_sub_menu = false;
        }
    }

    fn add_console_input(&mut self, c: char) {
        self.console_input.insert(self.cursor_position, c);
        self.cursor_position += 1; // カーソル位置を右に移動
    }

    fn remove_console_input(&mut self) {
        if self.cursor_position > 0 && !self.console_input.is_empty() {
            self.console_input.remove(self.cursor_position - 1);
            self.cursor_position -= 1; // カーソル位置を左に移動
        }
    }

    // カーソルを左に移動
    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    // カーソルを右に移動
    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.console_input.len() {
            self.cursor_position += 1;
        }
    }

    fn submit_message(&mut self) {
        // 入力された文字列をTextBoxに流す
        self.messages.push(self.console_input.clone());
        self.console_input.clear();
        self.cursor_position = 0; // カーソル位置をリセット
    }
}

// メイン関数
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // キーが押された瞬間だけ処理する
                if key.kind == KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::Normal => {
                            // 通常モード
                            match key.code {
                                KeyCode::Char(':') => {
                                    app.input_mode = InputMode::Editing;  // Console入力モードに変更
                                }
                                KeyCode::Esc => app.back_to_main_menu(),
                                KeyCode::Up => app.move_up(),
                                KeyCode::Down => app.move_down(),
                                KeyCode::Enter => app.select_item(),
                                KeyCode::Char('q') => break,  // 'q'で終了
                                _ => {}
                            }
                        }
                        InputMode::Editing => {
                            // 編集モード（Consoleへの入力）
                            match key.code {
                                KeyCode::Esc => app.input_mode = InputMode::Normal,  // 通常モードに戻る
                                KeyCode::Char(c) => app.add_console_input(c),  // 文字の入力
                                KeyCode::Backspace => app.remove_console_input(),  // バックスペースで削除
                                KeyCode::Enter => app.submit_message(),  // Enterでメッセージ送信
                                KeyCode::Left => app.move_cursor_left(),  // 左キーでカーソル移動
                                KeyCode::Right => app.move_cursor_right(),  // 右キーでカーソル移動
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
