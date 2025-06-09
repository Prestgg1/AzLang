pub fn map_type(typ: &str, is_const: bool) -> String {
    if typ.starts_with("siyahı<") && typ.ends_with(">") {
        let inner_type_start = "siyahı<".len();
        let inner_type_end = typ.len() - 1;
        let inner_type_str = &typ[inner_type_start..inner_type_end];
        let mapped_inner_type = map_type(inner_type_str, is_const);

        if is_const {
            format!("&[{}]", mapped_inner_type)
        } else {
            format!("Vec<{}>", mapped_inner_type)
        }
    } else {
        match typ {
            "ədəd" | "integer" => "usize".to_string(),
            "mətn" => {
                if is_const {
                    "&str".to_string()
                } else {
                    "String".to_string()
                }
            }
            _ => typ.to_string(),
        }
    }
}
