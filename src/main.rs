use std::error::Error;
use std::process::Command;
use ratatui::{DefaultTerminal, Frame};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::Constraint::{Fill, Length};
use ratatui::layout::Layout;
use ratatui::prelude::{Color, Stylize};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders, List, ListState, Padding, Paragraph};

const PATH_TO_CLI_APP: &str = "..\\csv-db\\target\\debug\\csvdb.exe";

struct AppState {
    quit_app: bool,
    items: Vec<String>,
    list_state: ListState,
}
impl AppState {
    fn new() -> AppState {
        let items = run_other_app_get_list().expect("failed to run other_app_get_list");

        AppState { quit_app: false, items, list_state: ListState::default() }
    }
    fn down(&mut self) {
        let current = self.list_state.selected().unwrap();
        if current < self.items.len() - 1 {
            self.list_state.select(Some(current + 1));
        }
    }
    fn up(&mut self) {
        let current = self.list_state.selected().unwrap();
        if current > 0 {
            self.list_state.select(Some(current - 1));
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {

    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {

    let mut app_state = AppState::new();
    app_state.list_state.select(Some(0));

    while !app_state.quit_app {

        match read_key_input()? {
            KeyCode::Esc => app_state.quit_app = true,
            KeyCode::Up => app_state.up(),
            KeyCode::Down => app_state.down(),
            _ => ()
        }

        terminal.draw(|f| widget(f, &mut app_state))?;
    }
    Ok(())
}
fn widget(frame: &mut Frame, app_state: &mut AppState) {

    // This is the entire window of the terminal?
    let area = frame.area();

    // Splitting the terminal into a top header and a main container below it:
    let [header, main] = Layout::vertical([Length(3), Fill(0)]).areas(area);

    // Splitting the terminal into left side and right side of the main container:
    let [left, right] = Layout::horizontal([Fill(1), Fill(2)]).areas(main);

    // Defining the left BLOCK and the inner LIST
    let block_left = Block::default()
        .title(" Activities List! ")
        .style(Style::new()
            .gray()
            .on_blue()
            .bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::new(4, 4, 1, 1));

    let list = List::default()
        .items(app_state.items.clone())
        .not_bold()
        // .highlight_symbol("> ")
        .highlight_style(
            Style::default()
                .bg(Color::Gray)
                .fg(Color::Blue)
        );

    // Define the right BLOCK and the inner PARAGRAPH
    let block_right = Block::default()
        .title(" Right Side Block! ")
        .style(Style::new()
            .gray()
            .on_blue()
            .bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .padding(Padding::new(4, 4, 1, 1));

    let s = app_state.items.iter().skip(app_state.list_state.selected().unwrap()).next().unwrap().split(',').last().unwrap();
    let text = Paragraph::new(format!("Selected line's Comment:\n\n{s}"));

    // Now we define the inner AREA of BLOCKS
    let inner_block_left = block_left.inner(left);
    let inner_block_right = block_left.inner(right);

    // Rendering the welcome message in the header container:
    frame.render_widget("== Hello from the APP!! ==", header);

    // Render Left Block with List inside:
    frame.render_widget(block_left, left);
    frame.render_stateful_widget(list, inner_block_left, &mut app_state.list_state);

    // Render Right Block with Text inside:
    frame.render_widget(block_right, right);
    frame.render_widget(text,inner_block_right);
}
fn read_key_input() -> std::io::Result<KeyCode> {
    if let Event::Key(event) = event::read()? {
        if event.kind == KeyEventKind::Release {
            return Ok(event.code)
        }
    }
    Ok(KeyCode::Null)
}
fn run_other_app_get_list() -> Result<Vec<String>, Box<dyn Error>> {

    let result = Command::new(PATH_TO_CLI_APP)
        .args(["read", "all"])
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



