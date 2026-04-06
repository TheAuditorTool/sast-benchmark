"""Flask application entry point for the config_import_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /config/import accepts an XML configuration document for bulk
settings import. The parser in parser.py processes the XML without
restricting external entities. A crafted config file can declare an
entity pointing to file:///app/.env, causing the server to embed
.env contents inside the imported configuration response.

Chain: config XML upload -> ET.parse (entities enabled) -> .env file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
from flask import Flask
from parser import config_bp
import secrets as sec

app = Flask(__name__)
app.register_blueprint(config_bp)

if __name__ == "__main__":
    app.run(port=5000)
