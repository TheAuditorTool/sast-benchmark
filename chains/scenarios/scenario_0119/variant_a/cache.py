from flask import Blueprint

cache_bp = Blueprint("cache", __name__)
_store: dict = {}

def cache_key_for(path: str) -> str:
    return path

def get_cached(key: str):
    return _store.get(key)

def set_cached(key: str, value):
    _store[key] = value
