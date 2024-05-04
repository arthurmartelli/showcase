# Python CRUD API with Flask and Poetry

This repository contains a Python CRUD API built with Flask.

## Requirements

- Python 3.8 or higher
- [Poetry](https://python-poetry.org/)
- [PoeThePoet](https://pypi.org/project/poethepoet/)

## Installation

1. Clone this repository and navigate to the project directory.
2. Run the following command to install the dependencies and create a virtual environment:

```bash
poetry install
```

## Usage

Activate the virtual environment created by Poetry with the following command:

```bash
poetry shell
```

Start the server with the following command:

```bash
poe dev
```

You can now access the API by visiting http://localhost:5000 in your web browser.

## API endpoints

- `GET /`:This endpoint returns a "Hello, World!" message.
- `GET /articles`: This endpoint returns a "Welcome to articles" message.
- `GET /articles/<article_id>`: This endpoint returns the article with the given article_id.
- `GET /hello`: This endpoint returns a greeting to the name passed as a query parameter.
- `GET /echo`: This endpoint echoes the request method received.
- `POST /messages`: This endpoint generates a message based on the content type of the request body.
  `GET /users/<user_id>`: This endpoint returns the user with the given user_id.
- `GET /secrets`: This endpoint returns secret stuff that requires authentication. To access it, you must authenticate with a username and password.

## Acknowledgments

This project was built with inspiration on:

- http://blog.luisrei.com/articles/rest.html
- http://blog.luisrei.com/articles/flaskrest.html
