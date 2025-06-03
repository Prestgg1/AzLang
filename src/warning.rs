use colored::*; // Rəngləri istifadə etmək üçün

#[derive(Debug)]
#[allow(dead_code)]
pub enum Warning {
    UnusedParam(String),
    UnusedFunction(String),
    DeprecatedSyntax(String),
    ShadowedVariable(String),
    UnusedVariable(String),
    PossibleInfiniteLoop,
    EmptyFunctionBody(String),
    MutableButNeverChanged(String),
}

impl Warning {
    pub fn to_string(&self) -> String {
        match self {
            Warning::UnusedParam(param) => format!(
                "{}: '{}' adlı dəyişən funksiyada istifadə olunmur. Əgər bu dəyişən vacib deyilsə, adını '_{}' kimi yazın və ya silin.",
                "⚠️  Diqqət".yellow().bold(),
                param.bold(),
                param
            ),
            Warning::UnusedFunction(name) => format!(
                "{}: Funksiya '{}' heç yerdə istifadə olunmur.",
                "⚠️  Xəbərdarlıq".yellow().bold(),
                name.bold()
            ),
            Warning::DeprecatedSyntax(syn) => format!(
                "{}: '{}' sintaksisi köhnədir və tövsiyə edilmir.",
                "⚠️  Köhnəlmiş".yellow().bold(),
                syn.bold()
            ),
            Warning::ShadowedVariable(var) => format!(
                "{}: '{}' adlı dəyişən əvvəlcədən mövcuddur və üst-üstə yazılıb.",
                "⚠️  Kölgələnmiş dəyişən".yellow().bold(),
                var.bold()
            ),
            Warning::UnusedVariable(var) => format!(
                "{}: '{}' adlı dəyişən elan olunub, amma istifadə olunmayıb.",
                "⚠️  İstifadəsiz dəyişən".yellow().bold(),
                var.bold()
            ),
            Warning::PossibleInfiniteLoop => format!(
                "{}: Sonsuz dövrə riski var, diqqətli ol!",
                "⚠️  Təhlükə".yellow().bold()
            ),
            Warning::EmptyFunctionBody(name) => format!(
                "{}: '{}' funksiyasının gövdəsi boşdur.",
                "⚠️  Boş funksiya".yellow().bold(),
                name.bold()
            ),
            Warning::MutableButNeverChanged(var) => format!(
                "{}: '{}' adlı dəyişən `dəyişən` ilə elan olunub, amma heç dəyişməyib.",
                "⚠️  İstifadəsiz dəyişən".yellow().bold(),
                var.bold()
            ),
        }
    }
}

pub fn warn(warning: Warning) {
    println!("{}", warning.to_string());
}
