"""Subdomain redirect validation -- SAFE variant.

is_safe_redirect_url() uses urlparse to extract the netloc and then checks
whether it equals example.com or ends with .example.com (with a leading dot),
preventing the evil-example.com bypass.

Chain: /go?url=https://evil-example.com -> netloc check fails -> 400 blocked
"""
from urllib.parse import urlparse

ALLOWED_DOMAIN = "example.com"


def is_safe_redirect_url(url):
    """Return True only for the root domain or proper subdomains."""
    if not url:
        return False
    parsed = urlparse(url)
    host = parsed.netloc.lower().split(":")[0]
    return host == ALLOWED_DOMAIN or host.endswith("." + ALLOWED_DOMAIN)
