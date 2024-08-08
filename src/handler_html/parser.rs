use scraper::Html;

pub fn html_parser_document(content: String) -> Html {
    Html::parse_document(&content)
}
