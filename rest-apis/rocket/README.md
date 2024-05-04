# Rocket RESTful API

This repository contains an experiment for a CRUD (Create, Read, Update, Delete) API for a TODO list app written in Rust using the Rocket framework. The purpose of this experiment is to evaluate the performance, reliability, and ease of use of the API.

## Requirements

- Rust 1.55.0 or later
- Rocket 0.5.0 or later

## Installation

To install the app, clone this repository to your local machine and run the following command:

```bash
cargo run
```

This will build and run the app on your machine. You can then access the API at http://localhost:8000/api.

## Endpoints

The following endpoints are available in the API:

- `GET /api/healthchecker` – This route will return a simple health checker JSON object.
- `GET /api/todos?<page>&<limit>` – This route will return a selected or paginated list of Todo items.
- `POST /api/todos", data = "<body>` – This route will add a new Todo item to the data store.
- `GET /api/todos/<id>` – This route will retrieve a single Todo item from the in-memory database and return it to the client.
- `PATCH /api/todos/<id>", data = "<body>` – This route will edit the fields of a Todo item in the data store.
- `DELETE /api/todos/<id>` – This route will delete a Todo item from the in-memory database.

For more information about each endpoint and its expected inputs and outputs, please refer to the API documentation.

## Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request with your changes.
