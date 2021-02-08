use crate::discord::interaction::{
    Interaction, InteractionApplicationCommandCallbackData, InteractionResponse,
    InteractionResponseType,
};

pub(crate) fn params(meta: &Interaction) -> InteractionResponse {
    let options = meta.data.as_ref().unwrap().options.as_ref().unwrap();

    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: format!("You entered:\n{}", options[0].value).to_string(),
        }),
    }
}
