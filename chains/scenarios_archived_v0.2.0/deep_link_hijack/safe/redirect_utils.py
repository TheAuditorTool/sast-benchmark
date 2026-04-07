"""Deep-link redirect validation -- SAFE variant.

is_allowed_scheme() permits only https:// and the application's own myapp:// scheme,
blocking javascript:, data:, and other dangerous schemes.

Chain: POST /deep-link {url: javascript:...} -> 400 (scheme not permitted) -> blocked
"""
from urllib.parse import urlparse

ALLOWED_SCHEMES = {"https", "myapp"}


def is_allowed_scheme(url):
    """Return True only for explicitly allowed URL schemes."""
    if not url:
        return False
    parsed = urlparse(url)
    return parsed.scheme in ALLOWED_SCHEMES
