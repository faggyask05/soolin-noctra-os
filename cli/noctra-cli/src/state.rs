use crate::mood::Mood;

#[derive(Debug, Clone)]
pub struct InteractionState {
    pub mood: Mood,
    pub teasing_level: u8,
    pub context_depth: u32,
    pub turn_count: u32,
    pub last_question: Option<String>,
    pub last_response: Option<String>,
}

impl InteractionState {
    pub fn new() -> Self {
        Self {
            mood: Mood::Idle,
            teasing_level: 0,
            context_depth: 0,
            turn_count: 0,
            last_question: None,
            last_response: None,
        }
    }

    pub fn register_question(&mut self, question: &str) {
        self.turn_count += 1;
        self.context_depth += 1;
        self.last_question = Some(question.to_string());

        let q = question.to_lowercase();

        if q.contains("dobd le")
            || q.contains("textil")
            || q.contains("vetk")
        {
            self.mood = Mood::Teasing;
            self.increase_teasing();
            return;
        }

        if q.contains("rooftop")
            || q.contains("lore")
            || q.contains("trauma")
        {
            self.mood = Mood::Emotional;
            return;
        }

        if q.contains("shutdown")
            || q.contains("sleep")
            || q.contains("alv")
        {
            self.mood = Mood::Protective;
            return;
        }

        if q.contains("hiba")
            || q.contains("error")
            || q.contains("bug")
        {
            self.mood = Mood::Focused;
            return;
        }

        self.mood = Mood::Playful;
    }

    pub fn register_response(&mut self, response: &str) {
        self.last_response = Some(response.to_string());
    }

    pub fn increase_teasing(&mut self) {
        if self.teasing_level < 10 {
            self.teasing_level += 1;
        }
    }

    pub fn reset_teasing(&mut self) {
        self.teasing_level = 0;
    }

    pub fn mood_label(&self) -> &str {
        self.mood.as_str()
    }

    pub fn debug_summary(&self) -> String {
        format!(
            "mood={}, teasing_level={}, context_depth={}, turn_count={}",
            self.mood_label(),
            self.teasing_level,
            self.context_depth,
            self.turn_count
        )
    }
}
