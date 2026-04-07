"""Server version exploit chain entry point.

This file is IDENTICAL between vuln/ and safe/ variants.

The debug.py middleware adds Server: Flask/2.0.1 Python/3.9.0 to every
response.  An attacker uses this to identify applicable CVEs and craft
an exploit targeting the exact version.

CWE-200: Server version header enables targeted CVE exploitation.
Chain: Any response reveals Server version -> CVE identified -> exploit crafted
"""
from config import app
import debug  # noqa: F401 - registers after_request hook and /login route


if __name__ == "__main__":
    app.run(port=5000)
