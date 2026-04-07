import os
import hmac
from flask import session

def generate_csrf_token():
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]

def validate_csrf(token):
    expected = session.get("csrf_token", "")
    if not token or not expected:
        return False
    return hmac.compare_digest(expected, token)
