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

// struct AppState {
//     quit_app: bool,
//     items: Vec<String>,
//     user_input: String,
// }
// impl AppState {
//     fn new() -> AppState {
//         let items = run_other_app_get_list(&["read", "all"]).expect("failed to run other_app_get_list");
//
//         AppState { quit_app: false, items, user_input: String::new() }
//     }
//
//     }
//     fn parse_input(&mut self) {
//         if self.user_input.is_empty() {
//             return;
//         }
//         if !self.user_input.starts_with(":") {
//             return;
//         }
//
//         self.user_input.remove(0);
//         let slice = self.user_input.split_whitespace().collect::<Vec<&str>>();
//         self.items = slice.iter().map(|s| s.to_string()).collect();
//
//         match slice[0]  {
//             "exit" => self.quit_app = true,
//             "test" => self.items = slice[1..].iter().copied().map(String::from).collect::<Vec<String>>(),
//             "read" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
//             "remove" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
//             "add"  => {
//                 let mut iter = slice.iter();
//                 let mut head = iter.by_ref().take(3).map(|p| *p).collect::<Vec<_>>();
//                 let tail = iter.map(|p| *p).collect::<Vec<_>>().join(" ");
//                 head.push(&tail);
//                 self.items = run_other_app_get_list(&head).expect("failed to run other_app_get_list")
//             },
//             // "reindex" => self.items = run_other_app_get_list(&slice).expect("failed to run other_app_get_list"),
//             _ => {},
//         };
//         self.user_input = ":".to_string();
//     }
// }
fn main() -> Result<(), Box<dyn Error>> {
    let mut quit_app = false;
    let mut app = App::new();

    let mut terminal = ratatui::init();

    while !app.quit {

        match read_key_input() {
            Ok(KeyCode::Esc) => app.quit = true,
            Ok(KeyCode::Up) => app.left_block_list.up(),
            Ok(KeyCode::Down) => app.left_block_list.down(),
            // Ok(KeyCode::Char(input)) => { app_state.user_input.push(input); },
            // Ok(KeyCode::Backspace) => { app_state.user_input.pop(); },
            // Ok(KeyCode::Enter) => app_state.parse_input(),
            // Err(e) => {
            //     ratatui::restore();
            //     return Err(Box::new(e));
            // },
            _ => ()
        };

        // match terminal.draw(|f| window(f, &mut app_state)) {
        match terminal.draw(|f| window(f, &mut app)) {
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
// fn window(frame: &mut Frame, app_state: &mut AppState) {
fn window(frame: &mut Frame, app: &mut App) {

    // ******* WIDGETS *******

    // This is List for `left` area, encapsulated in a Block.
    let left_area_list = &mut app.left_block_list;

    // This is Paragraph for `right` area, encapsulated in a Block.
    let right_area_text = &app.right_block_paragraph;

    let command_block = &app.command_block;


    // ******* LAYOUT *******

    // This is the entire window / view of the terminal
    let area = frame.area();
    // Splitting the terminal into a top header, main container, and footer at the bottom
    let [header, main, footer] = Layout::vertical([Length(2), Fill(0), Length(3)]).areas(area);
    // Splitting the main part of area into left and right side
    let [left, right] = Layout::horizontal([Fill(1), Fill(2)]).areas(main);

    // ******* RENDERING *******

    // Rendering the welcome message on the Header Area:
    frame.render_widget("= App Header = Hello, World!", header);
    // Render List inside a Block on the Left Area:
    frame.render_stateful_widget(&left_area_list.widget, left, &mut left_area_list.list_state);
    // Render Paragraph inside a Block on the Right Area:
    frame.render_widget(&right_area_text.widget, right);
    // Render Input field on the Footer Area:
    frame.render_widget(&command_block.widget, footer);
}
/*
    ******************
    *** MY WIDGETS ***
    ******************
*/
struct App<'a> {
    // *** State *** //
    quit: bool,

    // ** Widgets ** //
    left_block_list: LeftBlockList<'a>,
    right_block_paragraph: RightBlockParagraph<'a>,
    command_block: CommandBlock<'a>,

}
impl<'a> App<'a> {
    fn new() -> App<'a> {
        let left_block_items = run_other_app_get_list(&["read", "all"]).expect("failed to run other_app_get_list");
        App {
            quit: false,
            left_block_list: LeftBlockList::new(left_block_items),
            right_block_paragraph: RightBlockParagraph::new(),
            command_block: CommandBlock::new(),
        }
    }
}
struct LeftBlockList<'a> {
    list_state: ListState,
    widget: List<'a>,
    focused: bool,
}
impl<'a> LeftBlockList<'a> {
    fn new(items: Vec<String>) -> LeftBlockList<'a> {
        let block = Block::default()
            .title(" Activities List! ")
            .style(Style::new()
                .gray()
                .on_blue()
                .bold())
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .padding(Padding::new(4, 4, 1, 1));

        let widget = List::default()
            .items(items.clone())
            .not_bold()
            // .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Blue))
            .block(block);

        LeftBlockList { widget, focused: true, list_state: ListState::default() }
    }
    fn down(&mut self) {
        // List does not properly constrains to last index
        // instead it goes to `len()` instead of `len()-1`
        let next = self.list_state.selected().map_or(0, |i| i + 1);
        // self.list_state.select(Some(next.min(self.items.len()-1)));
        self.list_state.select(Some(next));
    }
    fn up(&mut self) {
        // properly constrains to index 0.
        self.list_state.select_previous();
    }
}
struct RightBlockParagraph<'a> {
    widget: Paragraph<'a>,
}
impl<'a> RightBlockParagraph<'a> {
    fn new() -> RightBlockParagraph<'a> {
        let block = Block::default()
                .title(" Right Side Block! ")
                .style(Style::new()
                    .gray()
                    .on_blue()
                    .bold())
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .padding(Padding::new(4, 4, 1, 1));

            let s = "Default, Message...";
            // if aps.items.len() > 0 {
            //     let index = aps.list_state.selected().unwrap_or(0);
            //     let index = index.min(aps.items.len()-1);
            //     s = aps.items.iter().skip(index).next().unwrap().split(',').last().unwrap();
            // }
            let text = format!("Selected line's Comment:\n{s}");

        RightBlockParagraph { widget: Paragraph::new(text).block(block) }
    }
}
struct CommandBlock<'a> {
    widget: Paragraph<'a>,
    focused: bool,
}
impl<'a> CommandBlock<'a> {
    fn new() -> CommandBlock<'a> {
        let block = Block::bordered()
            .title(" Command ")
            .bold()
            .border_type(BorderType::Thick)
            .padding(Padding::new(1, 1, 0, 0));

        let widget = Paragraph::new(":lorem ipsum")
            .style(Style::default()
                .bg(Color::Gray)
                .fg(Color::Blue)
                .not_bold())
            .block(block);

        CommandBlock { widget, focused: false }
    }
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
const PATH_TO_CLI_APP: &str = "..\\csv-db\\target\\debug\\csvdb.exe";
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



