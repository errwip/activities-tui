use std::error::Error;
use std::process::{Command};
use ratatui::{Frame};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::{Layout};
use ratatui::prelude::{Color, Stylize};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, Padding, Paragraph};

const PATH_TO_CLI_APP: &str = "..\\csv-db\\target\\debug\\csvdb.exe";

struct AppState {
    quit_app: bool,
    items: Vec<String>,
    list_state: ListState,
    user_input: String,
}
impl AppState {
    fn new() -> AppState {
        let items = run_other_app_get_list(&["read", "all"]).expect("failed to run other_app_get_list");

        AppState { quit_app: false, items, list_state: ListState::default(), user_input: String::new() }
    }
    fn down(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap();
        if current < self.items.len() - 1 {
            self.list_state.select(Some(current + 1));
        }
    }
    fn up(&mut self) {
        if self.items.len() == 0 {
            return;
        }
        let current = self.list_state.selected().unwrap();
        if current > 0 {
            self.list_state.select(Some(current - 1));
        }
    }
    fn parse_input(&mut self) {
        if self.user_input.is_empty() {
            return;
        }
        if !self.user_input.starts_with(":") {
            return;
        }

        self.user_input.remove(0);
        let slice = self.user_input.split_whitespace().collect::<Vec<&str>>();

        match slice[0]  {
            "exit" => self.quit_app = true,
            "test" => self.items = slice[1..].iter().copied().map(String::from).collect::<Vec<String>>(),
            // "add"  => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
            "read" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
            // "remove" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
            // "reindex" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
            _ => {},
        };
        self.user_input = ":".to_string();
    }
}
fn main() -> Result<(), Box<dyn Error>> {

    let mut app_state = AppState::new();
    app_state.list_state.select(Some(0));

    let mut terminal = ratatui::init();

    while !app_state.quit_app {

        match read_key_input() {
            Ok(KeyCode::Esc) => app_state.quit_app = true,
            Ok(KeyCode::Up) => app_state.up(),
            Ok(KeyCode::Down) => app_state.down(),
            Ok(KeyCode::Char(input)) => { app_state.user_input.push(input); },
            Ok(KeyCode::Backspace) => { app_state.user_input.pop(); },
            Ok(KeyCode::Enter) => app_state.parse_input(),
            Err(e) => {
                ratatui::restore();
                return Err(Box::new(e));
            },
            _ => ()
        };

        match terminal.draw(|f| window(f, &mut app_state)) {
            Err(e) => {
                ratatui::restore();
                return Err(Box::new(e));
            }
            _ => { }
        };
    };
    ratatui::restore();
    Ok(())
}
fn window(frame: &mut Frame, app_state: &mut AppState) {
    // ******* LAYOUT *******

    // This is the entire window / view of the terminal
    let area = frame.area();
    // Splitting the terminal into a top header, main container, and footer at the bottom
    let [header, main, footer] = Layout::vertical([Length(2), Fill(0), Length(3)]).areas(area);
    // Splitting the main part of area into left and right side
    let [left, right] = Layout::horizontal([Fill(1), Fill(2)]).areas(main);

    // ******* WIDGETS *******

    // This is List for `left` area, encapsulated in a Block.
    let left_area_list = LeftBlockList(app_state.items.clone());
    // This is Paragraph for `right` area, encapsulated in a Block.
    let right_area_text = RightBlockParagraph(&app_state);

    // ******* RENDERING *******

    // Rendering the welcome message on the Header Area:
    frame.render_widget("= App Header = Hello, World!", header);
    // Render List inside a Block on the Left Area:
    frame.render_stateful_widget(left_area_list, left, &mut app_state.list_state);
    // Render Paragraph inside a Block on the Right Area:
    frame.render_widget(right_area_text, right);
    // Render Input field on the Footer Area:
    frame.render_widget(InputBlock(&app_state), footer);
}
/*
    ******************
    *** MY WIDGETS ***
    ******************
*/
#[allow(non_snake_case)]
fn LeftBlockList<'a>(items: Vec<String>) -> List<'a> {

    let block = Block::default()
        .title(" Activities List! ")
        .style(Style::new()
            .gray()
            .on_blue()
            .bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::new(4, 4, 1, 1));

    List::default()
        .items(items)
        .not_bold()
        // .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .fg(Color::Blue))
        .block(block)
}
#[allow(non_snake_case)]
fn RightBlockParagraph<'a>(aps: &AppState) -> Paragraph<'a> {

    let block = Block::default()
        .title(" Right Side Block! ")
        .style(Style::new()
            .gray()
            .on_blue()
            .bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::new(4, 4, 1, 1));

    let mut s = "Hello, World!";
    if aps.items.len() > 0 {
        s = aps.items.iter().skip(aps.list_state.selected().unwrap()).next().unwrap().split(',').last().unwrap();
    }
    let text = format!("Selected line's Comment:\n{s}\n{:?}", aps.list_state.selected());

    Paragraph::new(text)
        .block(block)
}
#[allow(non_snake_case)]
fn InputBlock(aps: &'_ AppState) -> Paragraph<'_> {

    Paragraph::new(aps.user_input.as_str())
        .style(Style::default()
            .bg(Color::Gray)
            .fg(Color::Blue)
            .not_bold())
        .block(Block::bordered()
            .title(" Input ")
            .bold()
            .border_type(BorderType::Thick)
            .padding(Padding::new(1, 1, 0, 0)))
}
/*
    ************************
    *** Helper Functions ***
    ************************
*/
fn read_key_input() -> std::io::Result<KeyCode> {
    if let Event::Key(event) = event::read()? {
        if event.kind == KeyEventKind::Release {
            return Ok(event.code)
        }
    }
    Ok(KeyCode::Null)
}
fn run_other_app_get_list(args: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {

    let result = Command::new(PATH_TO_CLI_APP)
        .args(args)
        .output()?;

    Ok(String::from_utf8(result.stdout)?
        .lines()
        .skip(1)
        .map(String::from)
        .collect::<Vec<_>>())
}


























/*

    BE RIGHT BACK

 */



