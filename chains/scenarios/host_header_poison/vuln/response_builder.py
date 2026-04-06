"""Response builder -- VULNERABLE variant.

Generates absolute URLs for navigation links using the Host header
directly from the request without validation.  An attacker can supply
an arbitrary Host value that ends up in cached HTML, poisoning every
subsequent visitor with links pointing to the attacker's domain.

Chain: attacker-controlled Host header -> reflected in HTML links ->
       links cached -> poisoned links served to all visitors.
Individual findings: host header injection (medium)
Chain finding: host header injection -> cache poisoning (high)
"""
from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)


def build_base_url() -> str:
    """Return scheme + host for building absolute links."""
    host = request.headers.get("Host", "localhost")
    return f"http://{host}"
