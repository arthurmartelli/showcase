use common::{data_map, prelude::*};
use data_parser;
use google_oauth;

const SCOPES: &[&str; 3] = &[
    "https://www.googleapis.com/auth/userinfo.profile", // get user name
    "https://www.googleapis.com/auth/userinfo.email",   // get user email
    "https://mail.google.com/",                         // send emails
];

// we do not add drive here because the parser asks if needed
// as there will be more parsers implemented

pub async fn generate_email_sender(auth: &Auth) -> Result<Email, DynError> {
    let token = auth.token(SCOPES).await?;
    let email = google_oauth::get_user_info(&token, "email").await?;
    let name = google_oauth::get_user_info(&token, "name").await?;
    let server = "smtp.gmail.com";

    let email_address = EmailAddress::new(name, email);
    let sender = Sender::new(email_address, token, server);

    let selected_template = "email";
    let subject = "Bienvenido a Olympus";
    let global_context: DataMap = data_map!(
        "title" => "Título!",
        "company" => "Olympus Gym!"
    );

    let template = Template::new(selected_template);
    let receivers = get_receivers(auth).await?;

    Ok(Email::new(
        sender,
        receivers,
        subject,
        template,
        global_context,
    ))
}

async fn get_receivers(auth: &Auth) -> Result<DataSet<Receiver>, DynError> {
    let spreadsheet_id = "1wtC59FnCK6TLrkoST0AxvmNGzv5IQEYkI_2qMne7H5I";
    let range = "Clients!A:E";
    let columns = 5;

    data_parser::google_sheets::get_data(&auth, spreadsheet_id, range, columns).await
}
