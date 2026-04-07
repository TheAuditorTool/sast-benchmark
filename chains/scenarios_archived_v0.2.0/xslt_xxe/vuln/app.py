"""Flask application entry point for the xslt_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /transform accepts an XML source document in the request body.
The handler in handler.py parses the XML to extract transformation
directives. Because the parser does not restrict external entities,
an attacker can embed an entity pointing to /etc/ssl/private/server.key
and have the private key contents returned in the transformation output.

Chain: XML source body -> ET.fromstring (entities enabled) -> private key read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import xslt_bp
import config

app = Flask(__name__)
app.register_blueprint(xslt_bp)

if __name__ == "__main__":
    app.run(port=5000)
