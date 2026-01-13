use dialoguer::Password;

#[allow(dead_code)]
pub fn password_input() -> Box<str> {
    let a = Password::new()
        .with_prompt("Password")
        .with_confirmation("Confirm password", "Passwords do not match")
        .interact()
        .expect("Failed to read password");
    Into::into(a)
}
