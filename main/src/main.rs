use std::fs;

fn main() {
    let html: String = match fs::read_to_string("lib/src/index.html") {
        Ok(content) => content,
        Err(e) => {
            eprint!("Failed to read HTML file: {}", e);
            panic!();
        }
    };
    let css: String = match fs::read_to_string("lib/src/style.css") {
        Ok(content) => content,
        Err(e) => {
            eprint!("Failed to read CSS file: {}", e);
            panic!();
        }
    };
    let rs: String = match fs::read_to_string("lib/src/main.rs") {
        Ok(content) => content,
        Err(e) => {
            eprint!("Failed to read Rust file: {}", e);
            panic!();
        }
    };

    let p_html: Vec<String> = html_parser(html);
    let p_css: Vec<String> = css_parser(css);
    let p_rs: Vec<String> = rs_parser(rs);

    todo!();
}

fn html_parser(html: String) -> Vec<String> {
    html.split_whitespace().map(String::from).collect()
}

fn css_parser(css: String) -> Vec<String> {
    css.split_whitespace().map(String::from).collect()
}

fn rs_parser(rs: String) -> Vec<String> {
    rs.split_whitespace().map(String::from).collect()
}
