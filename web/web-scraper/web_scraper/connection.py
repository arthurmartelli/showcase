import requests


def get_response(url: str):
    """Handles the connection to the url

    #### Args:
        url (str): URL to get the response from

    #### Returns:
        Response: GET request from URL, as HTML
    """

    #! add error handling
    return requests.get(url, allow_redirects=False)
