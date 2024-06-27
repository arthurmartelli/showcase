# Learning Axum by cloning X/Twitter

Creating a very basic of X/Twitter: a micro blogging platform

## Features

- [ ] Create a post
    - [ ] Limit the characters per post
    - [ ] Optionally, can be a response to a post
    - [ ] Optionally, can be a response to a response
    - [ ] Posts are stored in Postgres
    - [ ] Posts are validated before storing in database
- [ ] Get a list of all top level posts
    - [ ] Text
    - [ ] Likes
    - [ ] Paginate
- [ ] Get one post
    - [ ] Get it's responses
        - [ ] Paginate
    - [ ] Text
    - [ ] Likes
- [ ] Update post text
    - [ ] Text
    - [ ] Un-delete
- [ ] Delete post
    - [ ] Soft delete post

## Tech & Crates

- rustc v1.79.0
    - axum v0.7.5
    - dotenvy v0.15.7
    - eyre v0.6.12
    - tokio v1.38.0
        - -F net,rt-multi-thread,macros
    - tracing v0.1.40
    - tracing_subscriber v0.3.18
    - tower-http v0.5.2
        - -F tracing
    - sqlx
- postgres v16.3 inside docker
- cli (use `cargo install`)
    - sqlx-cli 0.7.4

## Setup

1. Create the `.env` from the [`.env_example`](./.env_example) file.

    ```sh
    cp .env_example .env
    ```

## Database

A docker compose file is included to spin up a Postgres database. If you have docker installed, you can use the command:

```sh
docker compose up -d
```

### Models

#### Posts

| PK  | FK  | Name      | Type         |
| --- | --- | --------- | ------------ |
| *   | *   | post_id   | serial       |
|     |     | parent_id | int          |
|     |     | text      | varchar(255) |
|     |     | likes     | int          |
