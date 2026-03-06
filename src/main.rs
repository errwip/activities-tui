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

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let mut terminal = ratatui::init();

    while !app.quit {

        app.parse_input(read_key_input()?);

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
    frame.render_stateful_widget(&left_area_list.draw(), left, &mut left_area_list.list_state);
    // Render Paragraph inside a Block on the Right Area:
    frame.render_widget(&right_area_text.draw(&left_area_list), right);
    // Render Input field on the Footer Area:
    frame.render_widget(&command_block.draw(), footer);
}
/*
    ******************
    *** MY WIDGETS ***
    ******************
*/
struct App {
    // *** State *** //
    quit: bool,

    // ** Widgets ** //
    left_block_list: LeftBlockList,
    right_block_paragraph: RightBlockParagraph,
    command_block: CommandBlock,

}
impl App {
    fn new() -> App {
        let left_block_items = run_other_app_get_list(&["read", "all"]).expect("failed to run other_app_get_list");
        App {
            quit: false,
            left_block_list: LeftBlockList::new(left_block_items),
            right_block_paragraph: RightBlockParagraph::new(),
            command_block: CommandBlock::new(),
        }
    }
    fn parse_input(&mut self, code: KeyCode) {
        if code == KeyCode::Esc {
            self.quit = true;
        }
        if self.left_block_list.focused {
            match code {
                KeyCode::Up => self.left_block_list.up(),
                KeyCode::Down => self.left_block_list.down(),
                _ => { }
            }
        }
        if self.command_block.focused {
            match code {
                KeyCode::Char(input) => { self.command_block.user_input.push(input); },
                KeyCode::Backspace => { self.command_block.user_input.pop(); },
                KeyCode::Enter => {
                    if self.command_block.user_input.starts_with(":") {
                        self.command_block.user_input.remove(0);
                        let slice = self.command_block.user_input.split_whitespace().collect::<Vec<&str>>();
                        match slice.first().copied() {
                            Some("exit") => self.quit = true,
                            Some("test") => CommandBlock::command_test(&slice, &mut self.left_block_list.items),
                            Some("read") => CommandBlock::command_read(&slice, &mut self.left_block_list.items),
                            Some("add") => CommandBlock::command_add(&slice),
                            Some("remove") => CommandBlock::command_remove(&slice),
                            Some("reindex") => CommandBlock::command_reindex(),
                            _ => (),
                        }
                        let s = format!(":{}", slice.join(" "));
                        if self.command_block.command_buffer.last() != Some(&s) {
                            self.command_block.command_buffer.push(s);
                        }
                        self.command_block.buffer_index = self.command_block.command_buffer.len();
                        self.command_block.user_input = ":".to_string();
                    }
                }
                KeyCode::PageUp => self.command_block.up(),
                KeyCode::PageDown => self.command_block.down(),
                _ => { }
            }
        }
    }
}
struct LeftBlockList {
    items: Vec<String>,
    list_state: ListState,
    focused: bool,
}
impl LeftBlockList {
    fn new(items: Vec<String>) -> LeftBlockList {
        LeftBlockList { items, list_state: ListState::default(), focused: true }
    }
    fn draw<'a>(&self) -> List<'a> {
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
            .items(self.items.clone())
            .not_bold()
            // .highlight_symbol("> ")
            .highlight_style(
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Blue))
            .block(block)
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
struct RightBlockParagraph { }
impl RightBlockParagraph {
    fn new() -> RightBlockParagraph {
        RightBlockParagraph {}
    }
    fn draw<'a>(&self, dependency: &LeftBlockList) -> Paragraph<'a> {
        let block = Block::default()
                .title(" Right Side Block! ")
                .style(Style::new()
                    .gray()
                    .on_blue()
                    .bold())
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .padding(Padding::new(4, 4, 1, 1));

            let mut s = "Default, Message...";
            if dependency.items.len() > 0 {
                let index = dependency.list_state.selected().unwrap_or(0);
                let index = index.min(dependency.items.len()-1);
                s = dependency.items.iter().skip(index).next().unwrap().split(',').last().unwrap();
            }
            let text = format!("Selected line's Comment:\n{s}");

        Paragraph::new(text).block(block)
    }
}
struct CommandBlock {
    user_input: String,
    command_buffer: Vec<String>,
    buffer_index: usize,
    focused: bool,
}
impl CommandBlock {
    fn new() -> CommandBlock {
        let text = String::from(":lorem");
        CommandBlock { user_input: text, focused: true, command_buffer: vec![], buffer_index: 0 }

    }
    fn draw<'a>(&self) -> Paragraph<'a> {

        let block = Block::bordered()
            .title(" Command ")
            .bold()
            .border_type(BorderType::Thick)
            .padding(Padding::new(1, 1, 0, 0));

        Paragraph::new(self.user_input.clone())
            .style(Style::default()
                .bg(Color::Gray)
                .fg(Color::Blue)
                .not_bold())
            .block(block)
    }
    fn command_test(slice: &Vec<&str>, buffer: &mut Vec<String>) {
        *buffer = slice[1..].iter().copied().map(String::from).collect::<Vec<String>>();
    }
    fn command_read(slice: &Vec<&str>, buffer: &mut Vec<String>) {
        *buffer = run_other_app_get_list(&slice).expect("failed to run other_app_get_list");
    }
    fn command_add(slice: &Vec<&str>) {
        let mut iter = slice.iter();
        let mut head = iter.by_ref().take(3).map(|p| *p).collect::<Vec<_>>();
        let tail = iter.map(|p| *p).collect::<Vec<_>>().join(" ");
        head.push(&tail);
        run_other_app_get_list(&head).expect("failed to run other_app_get_list");
    }
    fn command_remove(slice: &Vec<&str>) {
        run_other_app_get_list(&slice).expect("failed to run other_app_get_list");
    }
    fn command_reindex() {
        /* TODO */
        // Implement Reindex
        // Needs response to confirm reindexing
    }
    fn up(&mut self) {
        if self.buffer_index > 0 {
            self.buffer_index -= 1;
            self.user_input = self.command_buffer[self.buffer_index].clone();
        }
    }
    fn down(&mut self) {
        if self.buffer_index < self.command_buffer.len() {
            self.buffer_index += 1;
            if self.buffer_index == self.command_buffer.len() {
                self.user_input = ":".to_string();
            }
            else {
                self.user_input = self.command_buffer[self.buffer_index].clone();
            }
        }
    }
        // if self.buffer_index < self.command_buffer.len()-1 {
        // }
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



