use crate::consts;
use poise::serenity_prelude as serenity;



pub fn embed() -> serenity::CreateEmbed {
    let author = serenity::CreateEmbedAuthor::new("Quantum")
        .url("https://github.com/SkohTV/quantum/")
        .icon_url("https://raw.githubusercontent.com/SkohTV/SkohTV/refs/heads/main/images/projects/quantum.png"); // need to update
    
    let footer = serenity::CreateEmbedFooter::new(format!(
        "Running on version {}", consts::VERSION
    ));

    serenity::CreateEmbed::default()
        .author(author)
        .footer(footer)
}
