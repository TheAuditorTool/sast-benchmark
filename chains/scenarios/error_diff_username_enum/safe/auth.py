"""Brute-force attack using enumerated usernames.

This file is IDENTICAL between vuln/ and safe/ variants.

With valid usernames confirmed via the error oracle in debug.py, an
attacker proceeds to brute-force passwords for those accounts.

CWE-200: Username enumeration narrows brute-force attack surface.
Chain: POST /login returns user-specific error -> username confirmed -> password brute-forced
"""
from config import app
import debug  # noqa: F401 - registers /login route


if __name__ == "__main__":
    app.run(port=5000)
