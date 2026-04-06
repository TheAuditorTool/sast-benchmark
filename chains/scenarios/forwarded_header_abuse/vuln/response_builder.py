"""Response builder -- VULNERABLE variant.

Constructs the base URL for password-reset links from the Forwarded
header (RFC 7239) without validating the host component.  An attacker
can forge the Forwarded header to redirect password-reset links to
their own domain; the response is then cached.

Chain: attacker-controlled Forwarded header -> used in reset URL ->
       URL cached -> poisoned reset links served to future requests.
Individual findings: unvalidated Forwarded header (medium)
Chain finding: Forwarded header abuse -> cache poisoning of reset links (high)
"""
import re
from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)


def base_url_from_forwarded() -> str:
    """Extract the host from the Forwarded header to build a base URL."""
    forwarded = request.headers.get("Forwarded", "")
    match = re.search(r"host=([^;,\s]+)", forwarded)
    if match:
        return f"https://{match.group(1)}"
    return "https://example.com"
