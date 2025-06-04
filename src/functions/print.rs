use lazy_static::lazy_static;
use regex::Regex;

/// çap(...) funksiyasını Rust koduna çevirir
pub fn handle_print(content: &str) -> String {
    lazy_static! {
        static ref TEMPLATE_RE: Regex = Regex::new(r#"^`(.*)`$"#).unwrap();
        static ref VAR_RE: Regex = Regex::new(r#"\$\{([^}]+)\}"#).unwrap();
    }

    let trimmed = content.trim();
    let cleaned = trimmed.trim_matches(['(', ')']).trim();

    if let Some(caps) = TEMPLATE_RE.captures(cleaned) {
        let inner = caps.get(1).unwrap().as_str();

        let mut result = String::new();
        let mut args = Vec::new();
        let mut last_end = 0;

        for caps in VAR_RE.captures_iter(inner) {
            let m = caps.get(0).unwrap();
            let expr = caps.get(1).unwrap().as_str().trim();

            result.push_str(&inner[last_end..m.start()]);
            result.push_str("{}");
            args.push(expr.to_string());
            last_end = m.end();
        }

        result.push_str(&inner[last_end..]);

        if args.is_empty() {
            format!("println!(\"{}\")", result)
        } else {
            format!("println!(\"{}\", {})", result, args.join(", "))
        }
    } else if Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$"#)
        .unwrap()
        .is_match(cleaned)
    {
        format!("println!(\"{{}}\", {})", cleaned)
    } else if cleaned.starts_with('"') && cleaned.ends_with('"') {
        format!("println!({})", cleaned)
    } else {
        format!("println!(\"{{}}\", {})", cleaned)
    }
}
