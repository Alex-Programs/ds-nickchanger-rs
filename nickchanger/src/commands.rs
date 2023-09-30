use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Vote for something
///
/// Enter `~vote pumpkin` to vote for pumpkins
#[poise::command(slash_command)]
pub async fn set_nick(
    ctx: Context<'_>,
    #[description = "Target"] target: serenity::Member,
    #[description = "New Nickname"] newnick: String,
) -> Result<(), Error> {
    ctx.guild()
        .unwrap()
        .edit_member(ctx.http(), target.user.id, |m| m.nickname(&newnick))
        .await
        .unwrap();

    let response = format!("{target} now has nickname {newnick}");
    ctx.reply(response).await?;
    Ok(())
}
