# Email Sender

## Overview

This project is an email sender application written in the Rust programming language. It uses OAuth2 for authentication and Handlebars for email templates. With this application, you can easily send emails to multiple recipients with a customizable template.

## Features

- Send emails using OAuth2 authentication
- Use Handlebars templates for email customization
- Support for multiple recipients
- Easy to use and integrate with other projects

## Requirements

- Rust programming language (version 1.45 or higher)
- A GMail account for OAuth2 authentication

## Setup

1. Clone this repository

   ```bash
   git clone https://github.com/yourusername/email-sender-rust.git
   ```

2. Change into the project directory

   ```bash
   cd email-sender-rust
   ```

3. Install dependencies

   ```bash
   cargo install
   ```

4. Create a GMail OAuth2 client ID and client secret. You can follow these instructions to create a GMail OAuth2 client ID and secret: https://developers.google.com/gmail/api/guides/authentication
5. Create a .env file in the project root and add the following variables:

   ```makefile
   SERVER =  smtp.gmail.com
   AUTHENTICATION_PORT = 43377
   CLIENT_SECRET = clientsecret.json # Path to your client secret
   CLIENT_CACHE = tokencache.json # Path to the stored information for login
   DOCKER_WORKING_DIR = /src/app
   ```

6. Compile and run the project

   ```bash
   cargo run
   ```

## Usage

Create a Handlebars template for your email. You can use the template.hbs file as a starting point.

Fill in the necessary information (recipients, subject, and message body) in `lib::generate_receivers()` (in the future, a data parser will be added).

Run the project

```bash
   cargo run
```

## Contributing

We welcome contributions to this project. If you have an idea for a new feature or have found a bug, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.
