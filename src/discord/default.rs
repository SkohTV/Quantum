use crate::consts;
use poise::serenity_prelude as serenity;

pub fn embed() -> serenity::CreateEmbed {
    let (name, pfp) = match consts::mode() {
        "dev" => ("Quantum Dev", "https://cdn.discordapp.com/avatars/1042497340352770142/17aa48868280edeb5edda826a0e49e48?size=1024"),
        "release" => ("Quantum", "https://cdn.discordapp.com/avatars/1033842126334742659/5235b0f44210455555f1685cac3580b9?size=1024"),
        err => panic!("Incorrect bot mode (not 'release' or 'dev') => {}", err)
    };

    let author = serenity::CreateEmbedAuthor::new(name)
        .url("https://github.com/SkohTV/quantum/")
        .icon_url(pfp);

    let footer =
        serenity::CreateEmbedFooter::new(format!("Running on version {}", consts::version()));

    serenity::CreateEmbed::default()
        .author(author)
        .footer(footer)
}
