"""Flask application entry point for the soap_body_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /soap accepts a SOAP envelope and dispatches to handler.py. The
Content-Type is expected to be text/xml or application/soap+xml. An
attacker can send a SOAP body that contains an external entity pointing
to file:///etc/shadow; the vulnerable handler resolves it and returns
the file contents embedded in the parsed response.

Chain: SOAP body -> ET.fromstring (entities enabled) -> file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import soap_bp
import config

app = Flask(__name__)
app.register_blueprint(soap_bp)

if __name__ == "__main__":
    app.run(port=5000)
