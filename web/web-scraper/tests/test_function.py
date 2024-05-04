from web_scraper import connection


def test_connection():
    url = "https://www.python.org"
    response = connection.get_response(url)

    assert response.status_code == 200
    assert b"Python is a programming language" in response.content
