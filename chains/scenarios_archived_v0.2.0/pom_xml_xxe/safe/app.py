"""Flask application entry point for the pom_xml_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /build/analyze accepts a Maven POM (pom.xml) body and extracts
dependency metadata. The parser in handler.py processes it as XML
without disabling external entities. An attacker can submit a crafted
POM containing an entity that reads /run/secrets/ci_token and embeds
the CI/CD secret in the dependency list response.

Chain: POM XML body -> ET.fromstring (entities enabled) -> CI secret read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import pom_bp
import config

app = Flask(__name__)
app.register_blueprint(pom_bp)

if __name__ == "__main__":
    app.run(port=5000)
