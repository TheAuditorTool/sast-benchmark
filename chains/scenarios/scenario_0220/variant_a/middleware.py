from flask import request

def apply_cors(response):
    origin = request.headers.get("Origin", "")
    if origin.endswith(".example.com") or origin == "https://example.com":
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, POST, OPTIONS"
    return response
