from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}

def cache_key() -> str:
    cookie = request.headers.get("Cookie", "")
    return f"{request.path}|{cookie}"

def get_cached(key: str):
    return _store.get(key)

def set_cached(key: str, value):
    _store[key] = value
