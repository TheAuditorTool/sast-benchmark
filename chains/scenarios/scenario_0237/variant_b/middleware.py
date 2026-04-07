import re
from flask import request

_ORIGIN_RE = re.compile(r"^https://app\.example\.com$")

def apply_cors(response):
    origin = request.headers.get("Origin", "")
    if origin and _ORIGIN_RE.match(origin):
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
