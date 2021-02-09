use crate::discord::interaction::{
    Interaction, InteractionApplicationCommandCallbackData, InteractionResponse,
    InteractionResponseType,
};

pub(crate) fn params(meta: &Interaction) -> InteractionResponse {
    let options = meta.options().unwrap();

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: format!("You entered:\n{}", options[0].value).to_string(),
        }),
    }
}
