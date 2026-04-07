from flask import request

def get_effective_method() -> str:
    override = request.headers.get("X-HTTP-Method-Override", "")
    if override:
        return override.upper()
    return request.method

def apply_cors(response):
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
        response.headers["Access-Control-Allow-Methods"] = "GET"
    return response
