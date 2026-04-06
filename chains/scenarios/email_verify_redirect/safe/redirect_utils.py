"""Redirect validation for email verification -- SAFE variant.

is_safe_redirect_url() accepts only relative paths or same-host absolute URLs.

Chain: GET /verify?token=valid&next=https://evil.com -> is_safe_redirect_url rejects -> /dashboard
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
