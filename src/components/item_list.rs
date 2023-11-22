use rss::Item;
use rss::Channel;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;

/// A list of all items in one channel
#[derive(Default)]
pub struct ItemList<'ch> {
    command_tx: Option<UnboundedSender<Action>>,
    items: Vec<Item>,
    channel: &'ch Channel, // as long as the channel lives, so does the ItemList
}
