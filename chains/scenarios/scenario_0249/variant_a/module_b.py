from urllib.parse import urlparse

ALLOWED_HOSTS = {"example.com", "www.example.com", "docs.example.com"}

def is_allowed_destination(url):
    if not url:
        return False
    parsed = urlparse(url)
    return parsed.netloc in ALLOWED_HOSTS
