from flask import request, jsonify
import functools

def rate_limit(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        
        return f(*args, **kwargs)
    return decorated
