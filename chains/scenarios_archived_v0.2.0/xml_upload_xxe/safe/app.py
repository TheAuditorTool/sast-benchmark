"""Flask application entry point for the xml_upload_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /upload accepts a multipart XML file upload. The file is saved
temporarily and then passed to parser.py for processing. An attacker
can upload an XML document containing an external entity declaration
pointing to file:///etc/passwd; the default parser resolves it and
the file contents are returned in the parsed response.

Chain: XML file upload -> ET.parse with entities enabled -> local file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
from flask import Flask
from parser import parser_bp
import secrets as cfg

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = cfg.UPLOAD_FOLDER
os.makedirs(cfg.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(parser_bp)

if __name__ == "__main__":
    app.run(port=5000)
