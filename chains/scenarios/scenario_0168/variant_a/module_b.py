from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)

RESOURCE_ETAG = "v1-abc123"
RESOURCE_BODY = b"Current resource content"

def handle_conditional(response):
    client_etag = request.headers.get("If-None-Match", "")
    if client_etag == RESOURCE_ETAG:
        from flask import make_response
        not_modified = make_response("", 304)
        not_modified.headers["ETag"] = RESOURCE_ETAG  
        return not_modified
    response.headers["ETag"] = RESOURCE_ETAG
    return response
