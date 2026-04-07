"""Flask application entry point for the saml_response_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /saml/acs accepts a base64-encoded SAML Response from the IdP and
forwards it to handler.py for assertion extraction. Because the parser
does not disable external entity resolution, an attacker can craft a
SAML Response containing an entity declaration that reads /etc/shadow
and includes its contents as an attribute value in the assertion.

Chain: SAML Response -> base64 decode -> ET.fromstring (entities enabled) -> /etc/shadow read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import saml_bp
import config

app = Flask(__name__)
app.secret_key = config.SESSION_SECRET
app.register_blueprint(saml_bp)

if __name__ == "__main__":
    app.run(port=5000)
