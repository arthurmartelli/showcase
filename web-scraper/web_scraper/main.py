# !/usr/bin/env python3

# scrape the titles and URLs of the latest articles on the TechCrunch website.

from bs4 import BeautifulSoup
from . import format_print, connection


URL: str = "https://techcrunch.com/"


def main() -> None:
    """Goes trough the TechCrunch website and gets
    the title and link of the recent news"""

    response = connection.get_response(URL)

    soup = BeautifulSoup(response.content, "html.parser")
    articles = soup.find_all(
        # Define the thing it's looking for
        name="header",
        attrs={"class": "post-block__header"},
    )

    titles = [
        article.find("a", class_="post-block__title__link").text.strip()
        for article in articles
    ]
    urls = [
        article.find("a", class_="post-block__title__link")["href"]
        for article in articles
    ]

    format_print.print_colored(titles, urls)


if __name__ == "__main__":
    main()
