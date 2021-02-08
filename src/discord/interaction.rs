use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::discord::command::handle_command;
use crate::error::Error;

#[derive(Deserialize_repr)]
#[repr(u8)]
enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub(crate) enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) avatar: String,
    pub(crate) discriminator: String,
    pub(crate) public_flags: u128,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct GuildMember {
    pub(crate) user: Option<User>,
    pub(crate) roles: Vec<String>,
    pub(crate) permissions: String,
    pub(crate) pending: bool,
    pub(crate) mute: bool,
    pub(crate) nick: String,
    pub(crate) joined_at: String,
    pub(crate) is_pending: bool,
    pub(crate) deaf: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct InteractionOptionsData {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct ApplicationCommandInteractionData {
    pub(crate) name: String,
    pub(crate) id: String,

    pub(crate) options: Option<Vec<InteractionOptionsData>>,
}

#[derive(Serialize)]
pub(crate) struct InteractionApplicationCommandCallbackData {
    pub(crate) content: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub(crate) struct Interaction {
    #[serde(rename = "type")]
    ty: InteractionType,
    pub(crate) data: Option<ApplicationCommandInteractionData>,

    pub(crate) guild_id: String,
    pub(crate) id: String,
    pub(crate) channel_id: String,
    pub(crate) token: String,
    pub(crate) member: Option<GuildMember>,
    pub(crate) version: u8,
}

impl Interaction {
    fn data(&self) -> Result<&ApplicationCommandInteractionData, Error> {
        Ok(self
            .data
            .as_ref()
            .ok_or_else(|| Error::InvalidPayload("data not found".to_string()))?)
    }
}

#[derive(Serialize)]
pub(crate) struct InteractionResponse {
    #[serde(rename = "type")]
    pub(crate) ty: InteractionResponseType,
    pub(crate) data: Option<InteractionApplicationCommandCallbackData>,
}

impl Interaction {
    pub(crate) fn perform(&self) -> Result<InteractionResponse, Error> {
        Ok(match self.ty {
            InteractionType::Ping => InteractionResponse {
                ty: InteractionResponseType::Pong,
                data: None,
            },
            InteractionType::ApplicationCommand => handle_command(self.data()?, &self),
        })
    }
}
