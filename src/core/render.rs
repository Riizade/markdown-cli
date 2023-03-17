use markdown;

pub fn render_to_html(markdown: String, css: Option<String>, allow_embedded_html: bool) -> String {
    let mut options = markdown::Options::default();
    options.compile.allow_dangerous_html = allow_embedded_html;
    options.parse.constructs.gfm_table = true;
    let mut rendered_text = markdown::to_html_with_options(&markdown, &options).unwrap();

    let mut head: String = "<head>\n".to_string();

    if let Some(css_text) = css {
        head = format!("{head}<style>\n{css_text}\n</style>\n");
    }

    head = format!("{head}</head>\n");

    rendered_text = format!("{head}{rendered_text}");

    rendered_text
}
