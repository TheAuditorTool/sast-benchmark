"""Flask application entry point for the sitemap_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /sitemap/import accepts a sitemap XML body to bulk-import URLs.
The parser in handler.py processes the XML without disabling external
entities. An attacker can supply a sitemap that declares an entity
referencing file:///app/.env or similar credential files; the parser
resolves the entity and the contents are embedded in the URL list.

Chain: sitemap XML body -> ET.fromstring (entities enabled) -> file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import sitemap_bp
import config

app = Flask(__name__)
app.register_blueprint(sitemap_bp)

if __name__ == "__main__":
    app.run(port=5000)
