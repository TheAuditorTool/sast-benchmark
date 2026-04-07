"""Flask application entry point for the docx_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /document/upload accepts a DOCX file (a ZIP archive of XML parts).
The handler in parser.py unpacks the ZIP and parses word/document.xml.
Because the default ElementTree parser is used, a crafted document.xml
with external entities can read server-local files and return their
contents in the extracted document body.

Chain: DOCX upload -> ZIP extraction -> ET.parse (entities enabled) -> file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
from flask import Flask
from parser import docx_bp
import config

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.UPLOAD_FOLDER
os.makedirs(config.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(docx_bp)

if __name__ == "__main__":
    app.run(port=5000)
