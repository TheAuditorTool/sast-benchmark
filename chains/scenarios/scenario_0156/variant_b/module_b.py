from urllib.parse import urlparse

ALLOWED_HOST = "shop.example.com"

def is_safe_return_url(url):
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
