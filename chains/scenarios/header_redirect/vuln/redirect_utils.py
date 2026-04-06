"""Host-header redirect construction -- VULNERABLE variant.

build_redirect_url() reads the Host header directly from the request to
construct an absolute URL.  An attacker who controls the Host header can
cause the application to redirect the user to an arbitrary domain.

Chain: request with Host: evil.com -> /go?to=/path -> redirect to https://evil.com/path
"""
from flask import request


def build_redirect_url(path):
    """Build an absolute redirect URL using the Host header.

    BUG: trusts the Host header without validation.
    """
    host = request.headers.get("Host", "localhost")  # VULN: attacker-controlled
    return f"https://{host}{path}"
