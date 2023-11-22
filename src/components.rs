use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use crossterm::event::MouseEvent;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::config::Config;
use crate::tui::Event;
use crate::tui::Frame;

pub mod home;
pub mod rss_list;
pub mod item_list;

pub trait Component {
    #[allow(unused_variables)]
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        Ok(())
    }

    fn init(&mut self, area: Rect) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };
        Ok(r)
    }

    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()>;
}
