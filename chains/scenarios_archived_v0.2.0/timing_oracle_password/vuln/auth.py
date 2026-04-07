"""Login endpoint bootstrap.

This file is IDENTICAL between vuln/ and safe/ variants.

The /login route is defined in debug.py and imported here to register
it with the Flask app before startup.

CWE-200: Timing oracle on password comparison enables credential enumeration.
Chain: POST /login with crafted payloads -> timing side-channel -> password recovered -> login succeeds
"""
from config import app
import debug  # registers /login route


if __name__ == "__main__":
    app.run(port=5000)
