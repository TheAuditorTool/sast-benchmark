"""Redirect validation helpers -- SAFE variant.

is_safe_redirect_url() rejects any URL with a netloc that differs from the
application host, preventing open redirects after logout.

Chain: /logout?redirect=https://evil.com -> is_safe_redirect_url returns False -> /
"""
from urllib.parse import urlparse

ALLOWED_HOST = "app.example.com"


def is_safe_redirect_url(url):
    """Return True only for relative URLs or same-host absolute URLs."""
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
