use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use const_format::formatcp;

use crate::ui::{DrawCall, RenderQueue, UIWidget};
use crate::database::Database;
use crate::input::{Event, KeyEvent, RespondWithPage};
use crate::ui::router::Page;

pub struct Authenticate {}
const APP_NAME: &str = "Trellminal";
// public key
const API_KEY: &str = "bbc638e415942dcd32cf8b4f07f1aed9";

const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}&return_url=http://127.0.0.1:9999/auth");

const LOGO_XXS: &str = "_____|Trellminal Logo(But not really, Your Terminal is small)|_____";

const LOGO_XS: &str = "███████
█  █  █
█  ████
███████";

const LOGO_SM: &str = "███████████
██   █   ██
██   █   ██
██   ██████
███████████";

const LOGO_MD: &str = "
█████████████████
███    ███    ███
███    ███    ███
███    ███    ███
███    ██████████
███    ██████████
███    ██████████
█████████████████";

const LOGO_NR: &str = "█████████████████████
███      ███      ███
███      ███      ███
███      ███      ███
███      ███      ███
███      ███      ███
███      ████████████
███      ████████████
███      ████████████
█████████████████████
█████████████████████";

const LOGO_LR: &str = " █████████████████████████ 
███████████████████████████
████        ███        ████
████        ███        ████
████        ███        ████
████        ███        ████
████        ███        ████
████        ███        ████
████        ███████████████
████        ███████████████
████        ███████████████
███████████████████████████
███████████████████████████
 █████████████████████████ ";


const LOGO_XL: &str = " ████████████████████████████ 
██████████████████████████████
████         ████         ████
████         ████         ████
████         ████         ████
████         ████         ████
████         ████         ████
████         ████         ████
████         ████         ████
████         █████████████████
████         █████████████████
████         █████████████████
████         █████████████████
██████████████████████████████
██████████████████████████████
 ████████████████████████████ ";

fn logo(rect: &Rect) -> &'static str {
    if rect.width >= 31 && rect.height >= 16 {
        LOGO_XL
    } else if rect.width >= 28 && rect.height >= 14 {
        LOGO_LR
    } else if rect.width >= 21 && rect.height >= 11 {
        LOGO_NR
    } else if rect.width >= 17 && rect.height >= 9 {
        LOGO_MD
    } else if rect.width >= 11 && rect.height >= 5 {
        LOGO_SM
    } else if rect.width >= 7 && rect.height >= 4 {
        LOGO_XS
    } else {
        LOGO_XXS
    }
}

use async_trait::async_trait;
#[async_trait]
impl Page for Authenticate {
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let block = Block::default().title("Authenticate").borders(Borders::ALL);
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(rect);
        let center_rect = main_layout[1];
        let center_layout = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Length(1),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
            ])
            .split(center_rect);

        let logo = Paragraph::new(logo(&center_layout[0]))
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let title = Paragraph::new("Trellminal")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let text = Paragraph::new("In order to authentiate open this link in your browser,login with trello API and copy your access token here!")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let link = Paragraph::new(AUTH_URL)
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        vec![
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(logo), center_layout[0]),
            DrawCall::new(UIWidget::Paragraph(title), center_layout[1]),
            DrawCall::new(UIWidget::Paragraph(text), center_layout[2]),
            DrawCall::new(UIWidget::Paragraph(link), center_layout[3]),
        ]
    }

    async fn update(&mut self, event: Event<KeyEvent>, db: &mut Database) -> Option<String> {
        match event {
            Event::Input(event) => match event.code {
                _ => None,
            },
            Event::Request(req) => {
                let url = req.url();
                let token: &str = "token=";
                let hash_index = url.find("token=");
                if hash_index.is_some() {
                    let token: String = url.chars().skip(hash_index.unwrap_or(0) + token.len()).take(url.len() - token.len()).collect();
                    let fetch_user_url = format!("https://api.trello.com/1/members/me/?key={}&token={}", API_KEY, token);
                    let body = reqwest::get(fetch_user_url)
                        .await.ok().unwrap()
                        .text()
                        .await.ok();
                    req.respond_with_view("auth_success.html").unwrap();
                }
                None
            },
            Event::Tick => None,
        }
    }
}

impl Authenticate {
    pub fn new() -> Self {
        Self {}
    }
}
