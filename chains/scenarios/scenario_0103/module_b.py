from flask import request

ALLOWED_ORIGINS = {"https://app.example.com"}

def get_effective_method() -> str:
    return request.method

def apply_cors(response):
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET, DELETE, OPTIONS"
    return response
