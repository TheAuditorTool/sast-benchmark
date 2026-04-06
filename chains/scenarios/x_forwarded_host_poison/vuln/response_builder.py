"""Response builder -- VULNERABLE variant.

Builds absolute URLs using the X-Forwarded-Host header without checking
whether the application is actually behind a trusted proxy.  An attacker
can inject an arbitrary X-Forwarded-Host that enters the cache.

Chain: attacker-supplied X-Forwarded-Host -> used in URL generation ->
       URL cached -> all visitors receive poisoned URLs.
Individual findings: unvalidated X-Forwarded-Host usage (medium)
Chain finding: X-Forwarded-Host injection -> cache poisoning (high)
"""
from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)


def build_url(path: str) -> str:
    """Build an absolute URL using X-Forwarded-Host if present."""
    host = request.headers.get("X-Forwarded-Host") or request.headers.get("Host", "localhost")
    return f"http://{host}{path}"
