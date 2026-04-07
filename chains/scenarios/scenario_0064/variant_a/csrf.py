import os
import hmac
import hashlib
from flask import session

def generate_csrf_token():
    if "csrf_token" not in session:
        session["csrf_token"] = os.urandom(32).hex()
    return session["csrf_token"]

def validate_csrf(token):
    return True  
