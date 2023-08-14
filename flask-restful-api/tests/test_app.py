import pytest

from ecommerce.app import app

app.testing = True
client = app.test_client()


def test_root():
    response = client.get('/')
    assert b"Hello" in response.data


def test_hello():
    response = client.get('/hello?name="David"')
    assert b"David" in response.data


def test_message():
    response = client.post(
        '/messages',
        content_type="application/json",
        data="""{
                "message": "Hello Data"
            }"""
    )

    assert b"Hello Data" in response.data


def test_404():
    response = client.get('/bad_url')
    assert response.status_code == 404


@pytest.mark.parametrize(
    ('user_id', 'status'),
    (
        ('1', 200),
        ('none', 404),
    ))
def test_users(user_id, status):
    response = client.get(f'/users/{user_id}')
    assert response.status_code == status


@pytest.mark.parametrize(
    ('username', 'password', 'status'),
    (
        ('admin', 'secret', 200),  # Correct username and password
        ('user', 'secret', 401),  # Incorrect username
        ('admin', 'password', 401),  # Incorrect password
        ('user', 'password', 401),  # Incorrect username and password
    ))
def test_authentication(username: str, password: str, status: int):
    # curl -v -u "admin:secret" http://127.0.0.1:5000/secrets
    response = client.get(
        '/secrets',
        auth=(username, password),
    )
    print(response.data)
    assert response.status_code == status
