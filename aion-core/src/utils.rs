use crate::{AionError, AionResult};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

pub fn validate_version(version: &str) -> bool {
    let version_regex = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
    version_regex.is_match(version)
}

pub fn normalize_string(input: &str) -> String {
    input.trim().to_lowercase().replace(' ', "_")
}

pub fn hash_string(input: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn parse_priority(priority_str: &str) -> AionResult<u8> {
    match priority_str.to_lowercase().as_str() {
        "critical" => Ok(1),
        "high" => Ok(2),
        "medium" => Ok(3),
        "low" => Ok(4),
        "info" | "informational" => Ok(5),
        _ => priority_str.parse::<u8>().map_err(|_| {
            AionError::ValidationError {
                field: "priority".to_string(),
                message: format!("Invalid priority value: {}", priority_str),
            }
        }),
    }
}

pub fn format_duration(start: DateTime<Utc>, end: DateTime<Utc>) -> String {
    let duration = end.signed_duration_since(start);

    if duration.num_days() > 0 {
        format!("{} days", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes", duration.num_minutes())
    } else {
        format!("{} seconds", duration.num_seconds())
    }
}

pub fn merge_metadata(base: &HashMap<String, String>, overlay: &HashMap<String, String>) -> HashMap<String, String> {
    let mut result = base.clone();
    for (key, value) in overlay {
        result.insert(key.clone(), value.clone());
    }
    result
}

pub fn extract_keywords(text: &str) -> Vec<String> {
    let word_regex = Regex::new(r"\b[a-zA-Z]{3,}\b").unwrap();
    word_regex
        .find_iter(text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|word| !is_stop_word(word))
        .collect()
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word,
        "the" | "and" | "for" | "are" | "but" | "not" | "you" | "all" | "can" | "had" | "her" | "was" | "one" | "our" | "out" | "day" | "get" | "has" | "him" | "his" | "how" | "man" | "new" | "now" | "old" | "see" | "two" | "way" | "who" | "boy" | "did" | "its" | "let" | "put" | "say" | "she" | "too" | "use"
    )
}

pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "-_.".contains(*c))
        .collect()
}

pub fn calculate_similarity(text1: &str, text2: &str) -> f64 {
    let words1: std::collections::HashSet<_> = extract_keywords(text1).into_iter().collect();
    let words2: std::collections::HashSet<_> = extract_keywords(text2).into_iter().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

pub fn generate_checksum(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn is_valid_uuid(uuid_str: &str) -> bool {
    uuid::Uuid::parse_str(uuid_str).is_ok()
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub fn escape_sql_string(input: &str) -> String {
    input.replace('\'', "''").replace('\\', "\\\\")
}

pub fn truncate_string(input: &str, max_length: usize) -> String {
    if input.len() <= max_length {
        input.to_string()
    } else {
        format!("{}...", &input[..max_length.saturating_sub(3)])
    }
}

pub fn is_business_hours(timestamp: DateTime<Utc>) -> bool {
    let hour = timestamp.hour();
    hour >= 9 && hour < 17
}

pub fn generate_reference_id(prefix: &str) -> String {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S");
    let random_suffix: u32 = rand::random::<u32>() % 10000;
    format!("{}-{}-{:04}", prefix, timestamp, random_suffix)
}

use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn next_sequence_number() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random<T>() -> T
    where
        T: From<u64>,
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        T::from(hasher.finish())
    }
}