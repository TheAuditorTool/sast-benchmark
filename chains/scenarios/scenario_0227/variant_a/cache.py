from flask import Blueprint

cache_bp = Blueprint("cache", __name__)

def apply_cache_headers(response):
    response.headers["Cache-Control"] = "public, max-age=300"
    return response
