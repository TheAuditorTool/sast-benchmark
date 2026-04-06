"""Response builder -- VULNERABLE variant.

Builds Link preload headers from a user-supplied resource URL without
sanitising CRLF characters.  An attacker can inject additional headers
(including a fake response body) that get stored in the cache.

Chain: CRLF in Link header -> injected headers enter cache ->
       poisoned preload served to all subsequent visitors.
Individual findings: header injection via Link (medium)
Chain finding: Link header injection -> cache poisoning (high)
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_link_header(resource_url: str) -> str:
    """Return a Link preload header value for the given URL."""
    return f"<{resource_url}>; rel=preload; as=script"
