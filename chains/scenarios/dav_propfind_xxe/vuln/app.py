"""Flask application entry point for the dav_propfind_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

PROPFIND /dav/<path> accepts a WebDAV PROPFIND request body. WebDAV
uses XML for property exchange. The handler in handler.py parses the
body with the default ET parser. An attacker can send a PROPFIND body
with an external entity pointing to /etc/ssl/private/server.key; the
resolved entity content is embedded in the property response.

Chain: PROPFIND body -> ET.fromstring (entities enabled) -> private key read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import dav_bp
import config

app = Flask(__name__)
app.register_blueprint(dav_bp)

if __name__ == "__main__":
    app.run(port=5000)
