"""Referrer-leak redirect helpers -- SAFE variant.

is_safe_redirect_url() rejects external hosts, so the Referer header containing
?token= is never sent to a third-party server.

Chain: GET /share?token=SECRET&redirect=https://evil.com
       -> is_safe_redirect_url rejects -> fallback to / -> token never leaked
"""
from urllib.parse import urlparse

ALLOWED_HOST = "app.example.com"


def is_safe_redirect_url(url):
    """Return True only for relative or same-host URLs."""
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
