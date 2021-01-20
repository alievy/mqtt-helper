#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Data {
    topic: Option<String>,
    local_id: Option<String>,
}

impl Data {
    /// Create new Data object.
    pub fn new(topic: Option<String>, local_id: Option<String>) -> Self {
        Data { topic, local_id }
    }

    /// Fetch topic.
    pub fn topic(&self) -> &Option<String> {
        &self.topic
    }

    /// Fetch local id.
    pub fn local_id(&self) -> &Option<String> {
        &self.local_id
    }

    /// Check if topic exists.
    pub fn topic_is_found(&self, topic: &str) -> bool {
        match &self.topic {
            Some(value) if value == topic => true,
            Some(_) => false,
            None => false,
        }
    }
}
