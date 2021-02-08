mod hello;
mod params;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, Interaction, InteractionResponse, InteractionResponseType,
};

pub(crate) fn handle_command(
    data: &ApplicationCommandInteractionData,
    meta: &Interaction,
) -> InteractionResponse {
    match data.name.as_str() {
        "hello" => hello::hello(),
        "params" => params::params(&meta),
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
