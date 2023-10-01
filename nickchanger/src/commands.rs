use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Set someone's nick. No logs, no exceptions.
///
///
#[poise::command(slash_command)]
pub async fn set_nick(
    ctx: Context<'_>,
    #[description = "Target"] target: serenity::Member,
    #[description = "New Nickname"] newnick: String,
) -> Result<(), Error> {
    /*
    if target.user.id == ctx.author().id {
        ctx.send(|r| {
            r.content(format!(
                "You can't set your own username to `{newnick}`, you coward! Only other people can decide your name."
            ))
            .ephemeral(false)
        })
        .await?;
        return Ok(());
    }
    */

    if newnick.len() < 1 || newnick.len() > 32 {
        ctx.send(|r| {
            r.content("That username is too long - it must be between 1 and 32 chars")
                .ephemeral(true)
        })
        .await?;
        return Ok(());
    }

    ctx.guild()
        .unwrap()
        .edit_member(ctx.http(), target.user.id, |m| m.nickname(&newnick))
        .await?;

    let response = format!("{target} now has nickname `{newnick}`");
    ctx.send(|r| r.content(response).ephemeral(true)).await?;
    Ok(())
}
