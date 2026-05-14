#[derive(Debug, Clone, PartialEq)]
pub enum Mood {

    Idle,

    Playful,

    Focused,

    Teasing,

    Protective,

    Warning,

    Emotional,
}

impl Mood {

    pub fn as_str(&self) -> &str {

        match self {

            Mood::Idle => "idle",

            Mood::Playful => "playful",

            Mood::Focused => "focused",

            Mood::Teasing => "teasing",

            Mood::Protective => "protective",

            Mood::Warning => "warning",

            Mood::Emotional => "emotional",
        }
    }
}
