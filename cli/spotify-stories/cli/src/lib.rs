use dialoguer::Confirm;

pub fn ask_for_confirmation() -> bool {
    Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
        .expect("Error getting confirmation")
}
