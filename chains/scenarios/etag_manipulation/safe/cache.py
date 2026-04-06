"""Cache helper -- SAFE variant.

Only returns 304 when the client's If-None-Match exactly matches the
server-authoritative ETag; never echoes the client-supplied value.

Chain broken: ETag validated against server value -> attacker cannot
              cause the cache to store a crafted ETag
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)

RESOURCE_ETAG = "v1-abc123"
RESOURCE_BODY = b"Current resource content"


def handle_conditional(response):
    """Return 304 only when If-None-Match matches the server ETag."""
    client_etag = request.headers.get("If-None-Match", "")
    if client_etag == RESOURCE_ETAG:
        from flask import make_response
        not_modified = make_response("", 304)
        not_modified.headers["ETag"] = RESOURCE_ETAG  # always server value
        return not_modified
    response.headers["ETag"] = RESOURCE_ETAG
    return response
