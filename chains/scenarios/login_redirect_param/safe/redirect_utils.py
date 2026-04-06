"""Redirect validation helpers -- SAFE variant.

is_safe_redirect_url() accepts only relative URLs or URLs whose host
matches the application's own domain, blocking external redirects.

Chain: /login?next=https://evil.com -> is_safe_redirect_url returns False -> /dashboard
"""
from urllib.parse import urlparse

ALLOWED_HOST = "app.example.com"


def is_safe_redirect_url(url):
    """Return True only for same-host or relative URLs."""
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
