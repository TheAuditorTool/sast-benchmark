"""Cache helper -- VULNERABLE variant.

Accepts the ETag value supplied by the client in If-None-Match and echoes
it back as the ETag of the 304 response without validating it against the
actual resource ETag.  An attacker can supply a crafted ETag to cause the
cache to serve stale or attacker-chosen content.

Chain: user-supplied ETag accepted without validation ->
       crafted ETag stored in cache -> stale/wrong content served.
Individual findings: unvalidated ETag echo (low)
Chain finding: ETag manipulation -> cache serves stale content (medium/high)
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)

RESOURCE_ETAG = "v1-abc123"
RESOURCE_BODY = b"Current resource content"


def handle_conditional(response):
    """Return 304 if If-None-Match matches, echoing client ETag."""
    client_etag = request.headers.get("If-None-Match", "")
    if client_etag:
        from flask import make_response
        not_modified = make_response("", 304)
        not_modified.headers["ETag"] = client_etag  # reflects attacker value
        return not_modified
    response.headers["ETag"] = RESOURCE_ETAG
    return response
