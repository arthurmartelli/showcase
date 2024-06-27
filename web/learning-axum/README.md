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
