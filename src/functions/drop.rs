/// sil(...) funksiyasını Rust koduna çevirir
pub fn handle_drop(content: &str) -> String {
    let cleaned = content.trim().trim_matches(['(', ')']).trim();
    format!("drop({});", cleaned)
}
