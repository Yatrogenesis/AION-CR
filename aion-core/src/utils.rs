use crate::{AionError, AionResult};
use chrono::{DateTime, Utc, Timelike};
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
    let random_suffix: u32 = (::rand::random::<u32>() % 10000);
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(validate_email("user.name+tag@domain.co.uk"));
        assert!(validate_email("test123@test-domain.org"));

        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("test@"));
        assert!(!validate_email("test..test@domain.com"));
    }

    #[test]
    fn test_validate_version() {
        assert!(validate_version("1.0.0"));
        assert!(validate_version("10.5.23"));
        assert!(validate_version("0.0.1"));

        assert!(!validate_version("1.0"));
        assert!(!validate_version("1.0.0.1"));
        assert!(!validate_version("v1.0.0"));
        assert!(!validate_version("1.0.0-beta"));
    }

    #[test]
    fn test_normalize_string() {
        assert_eq!(normalize_string("  Hello World  "), "hello_world");
        assert_eq!(normalize_string("Test String"), "test_string");
        assert_eq!(normalize_string("UPPERCASE"), "uppercase");
        assert_eq!(normalize_string(""), "");
        assert_eq!(normalize_string("   "), "");
    }

    #[test]
    fn test_hash_string() {
        let hash1 = hash_string("test");
        let hash2 = hash_string("test");
        let hash3 = hash_string("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_parse_priority() {
        assert_eq!(parse_priority("critical").unwrap(), 1);
        assert_eq!(parse_priority("CRITICAL").unwrap(), 1);
        assert_eq!(parse_priority("high").unwrap(), 2);
        assert_eq!(parse_priority("medium").unwrap(), 3);
        assert_eq!(parse_priority("low").unwrap(), 4);
        assert_eq!(parse_priority("info").unwrap(), 5);
        assert_eq!(parse_priority("informational").unwrap(), 5);
        assert_eq!(parse_priority("1").unwrap(), 1);
        assert_eq!(parse_priority("10").unwrap(), 10);

        assert!(parse_priority("invalid").is_err());
        assert!(parse_priority("").is_err());
    }

    #[test]
    fn test_format_duration() {
        let start = Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap();

        let end_days = start + chrono::Duration::days(5);
        assert_eq!(format_duration(start, end_days), "5 days");

        let end_hours = start + chrono::Duration::hours(3);
        assert_eq!(format_duration(start, end_hours), "3 hours");

        let end_minutes = start + chrono::Duration::minutes(45);
        assert_eq!(format_duration(start, end_minutes), "45 minutes");

        let end_seconds = start + chrono::Duration::seconds(30);
        assert_eq!(format_duration(start, end_seconds), "30 seconds");
    }

    #[test]
    fn test_merge_metadata() {
        let mut base = HashMap::new();
        base.insert("key1".to_string(), "value1".to_string());
        base.insert("key2".to_string(), "value2".to_string());

        let mut overlay = HashMap::new();
        overlay.insert("key2".to_string(), "new_value2".to_string());
        overlay.insert("key3".to_string(), "value3".to_string());

        let result = merge_metadata(&base, &overlay);

        assert_eq!(result.get("key1"), Some(&"value1".to_string()));
        assert_eq!(result.get("key2"), Some(&"new_value2".to_string())); // Overlay wins
        assert_eq!(result.get("key3"), Some(&"value3".to_string()));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_extract_keywords() {
        let text = "The quick brown fox jumps over the lazy dog";
        let keywords = extract_keywords(text);

        assert!(keywords.contains(&"quick".to_string()));
        assert!(keywords.contains(&"brown".to_string()));
        assert!(keywords.contains(&"jumps".to_string()));
        assert!(!keywords.contains(&"the".to_string())); // Stop word
        assert!(!keywords.contains(&"and".to_string())); // Stop word
    }

    #[test]
    fn test_is_stop_word() {
        assert!(is_stop_word("the"));
        assert!(is_stop_word("and"));
        assert!(is_stop_word("for"));
        assert!(!is_stop_word("important"));
        assert!(!is_stop_word("regulation"));
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("Hello World!@#$%"), "Hello World");
        assert_eq!(sanitize_input("test_file-name.txt"), "test_file-name.txt");
        assert_eq!(sanitize_input("αβγ123"), "123");
        assert_eq!(sanitize_input(""), "");
    }

    #[test]
    fn test_calculate_similarity() {
        let text1 = "The quick brown fox";
        let text2 = "The fast brown dog";
        let text3 = "Completely different content";

        let similarity1 = calculate_similarity(text1, text2);
        let similarity2 = calculate_similarity(text1, text3);
        let similarity3 = calculate_similarity(text1, text1);

        assert!(similarity1 > 0.0);
        assert!(similarity2 >= 0.0);
        assert!(similarity3 > 0.0);
        assert!(similarity1 > similarity2);

        // Empty strings
        assert_eq!(calculate_similarity("", ""), 0.0);
    }

    #[test]
    fn test_generate_checksum() {
        let data1 = b"test data";
        let data2 = b"test data";
        let data3 = b"different data";

        let checksum1 = generate_checksum(data1);
        let checksum2 = generate_checksum(data2);
        let checksum3 = generate_checksum(data3);

        assert_eq!(checksum1, checksum2);
        assert_ne!(checksum1, checksum3);
        assert_eq!(checksum1.len(), 16); // 16 hex chars
    }

    #[test]
    fn test_is_valid_uuid() {
        assert!(is_valid_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_valid_uuid("6ba7b810-9dad-11d1-80b4-00c04fd430c8"));

        assert!(!is_valid_uuid("invalid-uuid"));
        assert!(!is_valid_uuid("550e8400-e29b-41d4-a716"));
        assert!(!is_valid_uuid(""));
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1536), "1.50 KB");
        assert_eq!(format_file_size(1048576), "1.00 MB");
        assert_eq!(format_file_size(1073741824), "1.00 GB");
        assert_eq!(format_file_size(1099511627776), "1.00 TB");
    }

    #[test]
    fn test_escape_sql_string() {
        assert_eq!(escape_sql_string("normal text"), "normal text");
        assert_eq!(escape_sql_string("text with 'quotes'"), "text with ''quotes''");
        assert_eq!(escape_sql_string("text with \\backslash"), "text with \\\\backslash");
        assert_eq!(escape_sql_string("text with 'quotes' and \\backslash"), "text with ''quotes'' and \\\\backslash");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("exactly_ten", 10), "exactly...");
        assert_eq!(truncate_string("this is a very long string", 10), "this is...");
        assert_eq!(truncate_string("test", 3), "");
        assert_eq!(truncate_string("", 10), "");
    }

    #[test]
    fn test_is_business_hours() {
        let morning = Utc.with_ymd_and_hms(2023, 1, 1, 9, 0, 0).unwrap();
        let afternoon = Utc.with_ymd_and_hms(2023, 1, 1, 14, 30, 0).unwrap();
        let evening = Utc.with_ymd_and_hms(2023, 1, 1, 18, 0, 0).unwrap();
        let night = Utc.with_ymd_and_hms(2023, 1, 1, 2, 0, 0).unwrap();

        assert!(is_business_hours(morning));
        assert!(is_business_hours(afternoon));
        assert!(!is_business_hours(evening));
        assert!(!is_business_hours(night));
    }

    #[test]
    fn test_generate_reference_id() {
        let id1 = generate_reference_id("TEST");
        let id2 = generate_reference_id("TEST");

        assert!(id1.starts_with("TEST-"));
        assert!(id2.starts_with("TEST-"));
        assert_ne!(id1, id2); // Should be different due to timestamp/random
        assert!(id1.len() > 10); // Should have reasonable length
    }

    #[test]
    fn test_next_sequence_number() {
        let num1 = next_sequence_number();
        let num2 = next_sequence_number();
        let num3 = next_sequence_number();

        assert!(num2 > num1);
        assert!(num3 > num2);
        assert_eq!(num2, num1 + 1);
        assert_eq!(num3, num2 + 1);
    }

    #[test]
    fn test_rand_random() {
        let rand1: u64 = rand::random();
        let rand2: u64 = rand::random();

        // Very unlikely to be equal
        assert_ne!(rand1, rand2);

        // Test u32 conversion
        let rand_u32: u32 = rand::random();
        assert!(rand_u32 <= u32::MAX as u32);
    }

    #[test]
    fn test_priority_error_message() {
        match parse_priority("invalid") {
            Err(AionError::ValidationError { field, message }) => {
                assert_eq!(field, "priority");
                assert!(message.contains("Invalid priority value: invalid"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_empty_metadata_merge() {
        let empty1: HashMap<String, String> = HashMap::new();
        let empty2: HashMap<String, String> = HashMap::new();

        let result = merge_metadata(&empty1, &empty2);
        assert!(result.is_empty());

        let mut base = HashMap::new();
        base.insert("key".to_string(), "value".to_string());

        let result = merge_metadata(&base, &empty2);
        assert_eq!(result.len(), 1);
        assert_eq!(result.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_extract_keywords_edge_cases() {
        assert!(extract_keywords("").is_empty());
        assert!(extract_keywords("the and for").is_empty()); // All stop words
        assert!(extract_keywords("a b c").is_empty()); // All too short

        let keywords = extract_keywords("Important regulation compliance framework");
        assert!(keywords.len() > 0);
        assert!(keywords.contains(&"important".to_string()));
        assert!(keywords.contains(&"regulation".to_string()));
    }
}