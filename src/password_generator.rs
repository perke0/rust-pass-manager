use rand::{Rng, thread_rng};

pub fn generate_password(length: usize, include_symbols: bool) -> String {
    let mut rng = thread_rng();
    let charset = if include_symbols {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
    } else {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    };

    let password: String = (0..length)
        .map(|_| {
            charset
                .chars()
                .nth(rng.gen_range(0..charset.len()))
                .unwrap()
        })
        .collect();

    password
}

#[allow(dead_code)]
pub fn check_password_strength(password: &str) -> (usize, String) {
    let mut score = 0;
    let mut suggestions = Vec::new();

    if password.len() >= 12 {
        score += 2;
    } else if password.len() >= 8 {
        score += 1;
    } else {
        suggestions.push("Use at least 8 characters");
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    } else {
        suggestions.push("Include uppercase letters");
    }

    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    } else {
        suggestions.push("Include lowercase letters");
    }

    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    } else {
        suggestions.push("Include numbers");
    }

    if password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
    {
        score += 2;
    } else {
        suggestions.push("Include special characters");
    }

    let strength = match score {
        0..=2 => "Weak",
        3..=4 => "Fair",
        5..=6 => "Good",
        7..=8 => "Strong",
        _ => "Very Strong",
    };

    let suggestion_text = if suggestions.is_empty() {
        "Great password!".to_string()
    } else {
        format!("Suggestions: {}", suggestions.join(", "))
    };

    (score, format!("{} - {}", strength, suggestion_text))
}
