"""Flask application entry point for the xliff_translation_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /translations/import accepts an XLIFF (XML Localisation Interchange
File Format) file for translation import. XLIFF is an XML dialect, so
parser.py parses it as XML. The vulnerable parser does not restrict
external entities, enabling an attacker to exfiltrate API keys stored
in environment-accessible files via crafted XLIFF entity declarations.

Chain: XLIFF upload -> ET.parse (entities enabled) -> API key file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
from flask import Flask
from parser import xliff_bp
import config

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.TRANSLATION_UPLOAD_FOLDER
os.makedirs(config.TRANSLATION_UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(xliff_bp)

if __name__ == "__main__":
    app.run(port=5000)
