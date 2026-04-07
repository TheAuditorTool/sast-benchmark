from flask import request

def build_redirect_url(path):
    host = request.headers.get("Host", "localhost")  
    return f"https://{host}{path}"
