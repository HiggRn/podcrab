use std::collections::HashMap;
use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use futures::StreamExt;
use ratatui::prelude::*;
use ratatui::widgets::*;
use rss::Channel;
use rss::ChannelBuilder;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use super::Frame;
use crate::action::Action;
use crate::config::Config;
use crate::config::KeyBindings;

/// A list of all RSS channels
#[derive(Default)]
pub struct RssList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    channel_list: Vec<Channel>, // list of RSS channels
}

impl Component for RssList {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let channel_name_list: Vec<_> = self
            .channel_list
            .iter()
            .map(Channel::title)
            .map(ListItem::new)
            .collect();
        let list = List::new(channel_name_list)
            .block(Block::default().title("RSS Feeds").borders(Borders::ALL));
        f.render_widget(list, area);
        Ok(())
    }
}

impl RssList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new channel to the list
    pub fn add(&mut self, title: String, url: String) {
        let channel = ChannelBuilder::default().title(title).link(url).build();
        self.channel_list.push(channel);
    }

    /// Update all RSS channels on the list
    pub async fn update_all(&mut self) -> Result<()> {
        let channel_len = self.channel_list.len();
        let update_all_success = true;
        let _old_channels: Vec<_> = futures::stream::iter(self.channel_list.iter_mut())
            .map(update_rss)
            .buffer_unordered(100)
            .collect::<Vec<_>>()
            .await
            .iter()
            .filter(move |&channel| -> Option<Channel> {
                if let Err(e) = channel {
                    eprintln!("{e}");
                    update_all_success = false;
                    false
                }
                true
            })
            .collect();
        if update_all_success {
            Ok(())
        } else {
            Err("RSS update error")
        }
    }

    /// Update one RSS channel
    pub async fn update(&mut self, channel_num: usize) -> Result<()> {
        let channel = self.channel_list.get_mut(channel_num)?;
        let _old_channel = update_rss(channel).await?;
        // TODO: Maybe I can add a feature highlighting all new items some day?
    }
}

/// Update one RSS channel and return the old channel
async fn update_rss(channel: &mut Channel) -> Result<Channel> {
    let link = channel.link();
    let content = reqwest::get(link).await?.bytes().await?;
    let new_channel = Channel::read_from(content)?;
    let old_channel = std::mem::replace(channel, new_channel);
    Ok(old_channel)
}
