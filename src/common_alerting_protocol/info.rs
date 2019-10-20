use chrono::prelude::*;

pub struct Info {
    event:       String,
    urgency:     String,
    severity:    String,
    certainty:   String,
    language:    String,
    audience:    String,
    effective:   DateTime<Utc>,
    onset:       DateTime<Utc>,
    expires:     DateTime<Utc>,
    sender_name: String,
    headlnie:    String,
    description: String,
    instruction: String,
    web:         String,
    contact:     String
}
