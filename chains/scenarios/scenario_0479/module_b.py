from urllib.parse import urlparse

ALLOWED_HOST = "app.example.com"

def is_safe_relay_state(url):
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
