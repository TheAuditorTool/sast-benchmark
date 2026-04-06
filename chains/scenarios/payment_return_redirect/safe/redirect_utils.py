"""Payment return URL validation -- SAFE variant.

is_safe_return_url() permits only URLs on the application's own domain.
Checkout attempts with external return_url values are rejected at creation time.

Chain: POST /checkout {return_url: https://evil.com} -> 400 (invalid return URL) -> blocked
"""
from urllib.parse import urlparse

ALLOWED_HOST = "shop.example.com"


def is_safe_return_url(url):
    """Return True only for relative or same-host return URLs."""
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
