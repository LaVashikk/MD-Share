use pulldown_cmark::{Options, Parser, html};

pub fn render_form() -> String {
    format!(
        include_str!("../assets/form.html"),
        styles = include_str!("../assets/styles.css")
    )
}

pub fn render_paste(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    format!(
        include_str!("../assets/view.html"),
        styles = include_str!("../assets/styles.css"),
        body = html_output,
        raw_content = content,
    )
}
