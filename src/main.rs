use buffer::Buffer;
#[allow(dead_code, unused)]
use core::panic;

use async_openai::types::{CreateMessageRequest, CreateRunRequest, MessageRole};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use dotenv::dotenv;
use layout::{Alignment, Rect};
use ratatui::*;
use serde_json::from_str;
use sqlx::postgres::PgPoolOptions;
use style::Stylize;
use symbols::border;
use text::{Line, Text};
use tuidui::{display::display_tasks, openai::*, save::load_tasks, task::*};
use widgets::{
    block::{Position, Title},
    Block, Paragraph, Widget,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);

    // setup the postgreSQL db
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://Haller@localhost/tuidui_db")
        .await?;
    ratatui::restore();
    test().await?;
    app_result
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), anyhow::Error> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<(), anyhow::Error> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Tui Dui ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

async fn test() -> Result<(), anyhow::Error> {
    println!("Hello Work!");

    // Laod environment variables
    dotenv().ok();

    //Create a client for OpenAI
    let Ok(client) = initialize().await else {
        panic!("The client is not valid");
    };
    //Create an assistant from the task_creator file
    let assistant = create_assistant("task_creator", &client).await?;
    // println!("Assistant created: ");
    // println!("{:#?}", assistant);
    let tasks = load_tasks()?;
    display_tasks(&tasks.tasks);

    let thread = create_thread(&client).await?;
    println!("{}", format!("Thread created: {:#?}", thread));
    let prompt = "Good morning, Minister. Here’s your brief for the day, with the tasks requiring your attention:

     	1.	Attend a briefing with the Council of Magical Creatures to discuss the recent surge in unicorn poaching incidents. They await your decision on whether to increase protective wards in enchanted forests.
     	2.	Approve the new regulations for potion-making licenses, particularly concerning the restriction of volatile ingredients in student potions classes. Professor Morgana has sent her recommendations.
     	3.	Meet with the Department of Magical Transportation to finalize the approval of a new Floo Network expansion, connecting remote wizarding villages to major magical cities.
     	4.	Oversee the enchantment upgrades for the Ministry’s Defense Wards. There have been reports of weakening in certain areas, and this needs your urgent review.
     	5.	Review and sign off on the Magical Education Reform Bill, ensuring the inclusion of non-traditional forms of magic like Wandless Casting and Potionless Healing into the curriculum.
     	6.	Hold a closed-door session with the Auror Command to discuss the increasing threat posed by rogue magical factions, and coordinate a response plan.
     	7.	Inspect the progress of the Time-Turner Research Unit as they work to stabilize safe time travel spells. There have been incidents of wizards disappearing for longer than expected.
     	8.	Host a diplomatic luncheon with the visiting delegation from the Enchanted Isles, aimed at strengthening alliances and negotiating the safe exchange of rare magical artifacts.
     	9.	Sign off on the relocation order for the Veela community, whose current habitat has been encroached upon by Muggle construction. The Department of Magical Creatures has prepared alternative sanctuary locations.
     	10.	Receive a progress report from the Spell Innovation Bureau, which has been tasked with developing new counter-curses for recent dark spell variants discovered in the northern territories.

     These matters require your immediate attention, Minister. Shall we begin with the Council of Magical Creatures?";
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let user = "Roland";
    let pre_prompt = format!("Date: {}\nUser: {}\nMessage: ", date, user);
    println!("Prompt: {prompt}");
    let _message = client
        .threads()
        .messages(&thread.id)
        .create(CreateMessageRequest {
            role: MessageRole::User,
            content: (pre_prompt + prompt).into(),
            ..Default::default()
        })
        .await?;
    let run = client
        .threads()
        .runs(&thread.id)
        .create(CreateRunRequest {
            assistant_id: assistant.id,
            ..Default::default()
        })
        .await?;

    if let Some(message) = poll_run(&client, &run).await? {
        let content = message.content.first().unwrap();
        let text = match content {
            async_openai::types::MessageContent::Text(content) => content.text.value.clone(),
            async_openai::types::MessageContent::ImageFile(_)
            | async_openai::types::MessageContent::ImageUrl(_) => {
                panic!("Images are not expected in this example");
            }
            async_openai::types::MessageContent::Refusal(refusal) => refusal.refusal.clone(),
        };

        // Deserialize the JSON string into the ApiResponse struct
        let response: Tasks = from_str(&text).expect("Invalid JSON response");

        // Iterate over tasks and print them
        // for task in response.tasks.clone() {
        //     println!("{}", format!("Parsed task: {:#?}", task));
        // }
        let tasks = response.tasks.to_vec();
        display_tasks(&tasks);
    }

    // let cancel_run = cancel_run(&client, &run).await?;
    let poll2 = poll_run(&client, &run).await?;
    // println!("poll2: {:#?}", poll2);

    let list = list_assistants(&client).await?;
    // println!("Assistants: {:#?}", list);
    for assistant_object in list.data {
        delete_assistant(assistant_object, &client).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() -> Result<(), anyhow::Error> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
