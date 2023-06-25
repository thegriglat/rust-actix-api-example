diesel::table! {
    urls (short_url) {
        url -> Text,
        short_url -> Text,
    }
}
