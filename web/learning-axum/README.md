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

- Rust v1.79.0
    - axum v0.7.5
    - sqlxcar
