from flask import request

ALLOWED_ORIGINS = {"https://app.example.com", "https://partner.example.com"}

def apply_cors(response):
    origin = request.headers.get("Origin", "")
    if origin in ALLOWED_ORIGINS:
        response.headers["Access-Control-Allow-Origin"] = origin
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
