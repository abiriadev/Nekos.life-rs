// use pretty_assertions::assert_eq;

use {
    crate::{UnitResult, BASEURL},
    pretty_assertions::assert_eq,
};

#[test]
fn string_to_url() -> UnitResult {
    use super::IntoUrl;

    Ok(assert_eq!(
        "Waifu".into_url()?.as_str(),
        format!(
            "{baseurl}img/waifu",
            baseurl = BASEURL.as_str()
        )
    ))
}

#[tokio::test]
async fn parse_test() -> UnitResult {
    Ok(assert!(lazy_regex::regex_is_match!(
        r"^https://cdn\.nekos\.life/neko/[\w_.]+$",
        &<&str as super::IntoUrl>::parse(
            reqwest::get(
                BASEURL.join("img/")?.join("neko")?
            )
            .await?
        )
        .await?
    )))
}
