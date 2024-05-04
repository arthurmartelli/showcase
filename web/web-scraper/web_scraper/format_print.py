from tabulate import tabulate
from termcolor import colored


def print_table(titles: list[str], urls: list[str]) -> None:
    """Prints the titles and urls scrapped in table format

    #### Args:
        - titles (list[str]): Titles scrapped from the URL
        - urls (list[str]): Urls for the articles
    """
    table = zip(titles, urls)
    print(tabulate(table, headers=["Title", "URL"], tablefmt="fancy_grid"))


def print_colored(titles: list[str], urls: list[str]) -> None:
    """Prints the titles and urls scrapped in colored in the terminal

    #### Args:
        - titles (list[str]): Titles scrapped from the URL
        - urls (list[str]): Urls for the articles
    """
    for title, url in zip(titles, urls):
        title = colored(title, "green", attrs=["bold"])
        url = colored(url, "light_blue", attrs=["underline"])

        print(f"- {title}: {url}")
