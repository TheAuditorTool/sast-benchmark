"""Short-URL redirect validation -- SAFE variant.

is_allowed_destination() permits only URLs whose host is in an explicit allowlist.
Attempts to shorten an external URL are rejected at creation time.

Chain: POST /shorten {url: https://evil.com} -> 400 (destination not allowed) -> blocked
"""
from urllib.parse import urlparse

ALLOWED_HOSTS = {"example.com", "www.example.com", "docs.example.com"}


def is_allowed_destination(url):
    """Return True only for URLs whose host is in the allowlist."""
    if not url:
        return False
    parsed = urlparse(url)
    return parsed.netloc in ALLOWED_HOSTS
