from functools import wraps
from typing import Callable

from flask import Flask, Response, json, jsonify, request

app = Flask(__name__)


@app.route('/')
def api_root() -> str:
    """Root of the server

    Returns:
        str: main application

    Example Use:
        `curl http://127.0.0.1:5000/`
    """
    return "Hello, World!"


@app.route('/articles')
def api_articles() -> str:
    """Server articles

    Returns:
        str: articles in the server

    Example Use:
        `curl http://12.0.0.1:5000/articles`
    """
    return 'Welcome to articles'


@app.route('/articles/<int:article_id>')
def api_article(article_id: int) -> str:
    """Get each article

    Returns:
        str: article

    Example Use:
        `curl http://127.0.0.1:5000/articles/1`
    """
    return f'You are reading: {article_id}'


@app.route("/hello")
def api_hello() -> Response:
    """Says hello to the name passed

    Returns:
        `Response`: Response saying hello to the name, and a number

    Example Use:
        `curl http://127.0.0.1:5000/hello?name="Louis"`
    """

    default_name = "John Doe"
    name = request.args.get("name", default_name)

    data = {
        'hello': name,
        'number': '3',
    }

    js = json.dumps(data)

    resp = Response(js, status=200, mimetype='application/json')
    resp.headers["Link"] = "https://github.com"

    return resp


@app.route('/echo', methods=['GET', 'POST', 'PATCH', 'PUT', 'DELETE'])
def api_echo() -> str:
    """Echoes the request method

    Returns:
        `str`: request method received as `"ECHO: {request.method}"`

    Example use:
        `curl -X PATCH http://127.0.0.1:5000/echo`
    """
    return f"ECHO: {request.method}"


@app.route("/messages", methods=["POST"])
def api_message() -> str:
    """Generates the message for the server

    Returns:
        `str`: requested message in the format

    Example Use:
        - `curl -H "Content-type: application/json" -X POST http://127.0.0.1:5000/messages -d '{"message":"Hello Data"}'`
        - `curl -H "Content-type: application/octet-stream" -X POST http://127.0.0.1:5000/messages --data-binary @message.bin`
    """
    content = request.headers.get("Content-Type", "0")
    data = request.data
    json_data = request.json

    if content == 'text/plain':
        return f"Text message: {data}"

    elif content == 'application/json':
        try:
            return f"JSON message: {json_data}"
        except:
            return f"Error: Object is not JSON serializable."

    elif content == 'application/octet-stream':
        with open("./files/binary", "wb") as f:
            f.write(data)
        return "Binary message written!"

    else:
        return "415 Unsupported Media Type ;)"


@app.errorhandler(404)
def not_found(error=None) -> Response:
    """Error 404 handler
    """
    message = {
        'status': 404,
        'message': f"Not Found: {request.url}",
    }
    resp = jsonify(message)
    resp.status_code = 404

    return resp


@app.route('/users/<int:user_id>', methods=['GET'])
def api_users(user_id: int) -> Response:
    """Handles users in the app

    Args:
        user_id (int): user id to be seen

    Returns:
        `Response`: user requested
    """
    users = {
        1: 'john',
        2: 'mike',
        3: 'bill',
    }

    if user_id in users:
        return jsonify({user_id: users.get(user_id)})
    else:
        return not_found()


def check_auth(username: str, password: str) -> bool:
    """Checks if the request is authenticated

    Args:
        username (str)
        password (str)
    """

    return username == 'admin' and password == 'secret'


def authenticate():
    message = {'message': "Authenticate."}
    resp = jsonify(message)

    resp.status_code = 401
    resp.headers['WWW-Authenticate'] = 'Basic realm="Example"'

    return resp


def requires_auth(f: Callable):
    """Wrapper function to require authentication

    Args:
        f (`Callable`): function to require authentication
    """
    @wraps(f)
    def decorated(*args, **kwargs):
        auth = request.authorization

        if not auth:
            return authenticate()

        username = str(auth.username) if auth.username is not None else ""
        password = str(auth.password) if auth.password is not None else ""
        authentication = check_auth(username, password)

        if not authentication:
            return authenticate()

        return f(*args, **kwargs)

    return decorated


@app.route('/secrets')
@requires_auth
def api_secrets() -> str:
    """Secret stuff

    Returns:
        `str`: secret stuff that requires auth

    Example Use:
        `curl -v -u "admin:secret" http://127.0.0.1:5000/secrets`
    """
    return "Shhh this is top secret spy stuff!"
