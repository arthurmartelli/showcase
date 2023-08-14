# Web Scraper

This is a simple Python web scraper that extracts the titles and URLs of the latest articles on the TechCrunch website. The scraper uses the requests library to make HTTP requests to the website and the BeautifulSoup library to parse the HTML content. The implementation looks into [TechCrunch](https://techcrunch.com).

## Usage

To use the scraper, simply run the `'.\web_scraper\main.py'` script from the command line:

```bash
python '.\web_scraper\main.py'
```

Or, if you have [poetry][poetry] installed

```bash
poetry run main
```

## Installation

To install the required libraries, run the following command:

```bash
pip install -r requirements.txt
```

Or, if you have [poetry][poetry] installed

```bash
poetry install
poetry build
```

This will install the requests and beautifulsoup4 libraries.

## Testing

The testing is done with [PyTest][pytest]. To run the tests, use the command

```bash
poetry run pytest
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

<!-- Link Definitions -->

[poetry]: https://python-poetry.org/
[pytest]: https://docs.pytest.org/
