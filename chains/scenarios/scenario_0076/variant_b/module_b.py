from urllib.parse import urlparse

ALLOWED_SCHEMES = {"https", "myapp"}

def is_allowed_scheme(url):
    if not url:
        return False
    parsed = urlparse(url)
    return parsed.scheme in ALLOWED_SCHEMES
