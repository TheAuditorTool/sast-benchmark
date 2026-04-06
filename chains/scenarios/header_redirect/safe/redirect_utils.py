"""Host-header redirect construction -- SAFE variant.

build_redirect_url() uses a hardcoded application base URL instead of trusting
the Host header.  An attacker-supplied Host header has no effect.

Chain: request with Host: evil.com -> /go?to=/path -> redirect to https://app.example.com/path
"""

APP_BASE_URL = "https://app.example.com"


def build_redirect_url(path):
    """Build an absolute redirect URL using the hardcoded application base URL."""
    return f"{APP_BASE_URL}{path}"
