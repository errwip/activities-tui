use std::error::Error;
use std::process::Command;
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
}
impl AppState {
    fn new() -> AppState {
        let items = run_other_app_get_list(&["read", "all"]).expect("failed to run other_app_get_list");

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

    let mut app_state = AppState::new();
    app_state.list_state.select(Some(0));

    let mut terminal = ratatui::init();

    while !app_state.quit_app {

        match read_key_input() {
            Ok(KeyCode::Esc) => app_state.quit_app = true,
            Ok(KeyCode::Up) => app_state.up(),
            Ok(KeyCode::Down) => app_state.down(),
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

    // This is the entire window / view of the terminal
    let area = frame.area();

    // Splitting the terminal into a top header and a main container below
    let [header, main, footer] = Layout::vertical([Length(1), Fill(0), Length(3)]).areas(area);

    // Splitting the main part of area into left and right side
    // Now we have Header up top and Left Right parts bellow it
    let [left, right] = Layout::horizontal([Fill(1), Fill(2)]).areas(main);

    // Defining the left BLOCK and the inner LIST
    // let block_left = LeftBlock();
    let left_area_list = LeftBlockList(app_state.items.clone());

    // Define the right BLOCK and the inner PARAGRAPH
    let right_area_text = RightBlockParagraph(&app_state);
    // let text_right = RightBlockParagraph(&app_state);

    // Now we define the inner AREA of BLOCKS
    // let inner_block_left = block_left.inner(left);
    // let inner_block_right = block_left.inner(right);

    // Rendering the welcome message in the header container:
    frame.render_widget("= App Header = Hello, World!", header);

    // Render Left Block with List inside:
    // frame.render_widget(block_left, left);
    frame.render_stateful_widget(left_area_list, left, &mut app_state.list_state);

    // Render Right Block with Text inside:
    frame.render_widget(right_area_text, right);
    // frame.render_widget(text_right, inner_block_right);
    
    frame.render_widget(InputBlock(), footer);
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

    let s = aps.items.iter().skip(aps.list_state.selected().unwrap()).next().unwrap().split(',').last().unwrap();
    let text = format!("Selected line's Comment:\n{s}");
    Paragraph::new(text)
        .block(block)
}
#[allow(non_snake_case)]
fn InputBlock<'a>() -> Paragraph<'a> {
    let input: String ="Type Something!: ".to_string();
    let mut character_index: usize;
    let mut messages: Vec<String>;

    Paragraph::new(input.clone())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::bordered().title("Input"))
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



