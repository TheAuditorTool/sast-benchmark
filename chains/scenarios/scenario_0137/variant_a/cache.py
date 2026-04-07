from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}

def get_cached(key: str):
    return _store.get(key.lower())

def set_cached(key: str, value):
    _store[key.lower()] = value

def cache_key() -> str:
    return request.path.lower()
