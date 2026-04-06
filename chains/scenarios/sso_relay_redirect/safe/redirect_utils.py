"""SSO relay-state redirect validation -- SAFE variant.

is_safe_relay_state() accepts only relative paths or same-host absolute URLs,
blocking the open redirect that can follow SSO authentication.

Chain: RelayState=https://evil.com -> is_safe_relay_state rejects -> /dashboard
"""
from urllib.parse import urlparse

ALLOWED_HOST = "app.example.com"


def is_safe_relay_state(url):
    """Return True only for relative or same-host relay state URLs."""
    if not url:
        return False
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return False
    return True
