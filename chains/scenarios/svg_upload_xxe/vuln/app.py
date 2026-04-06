"""Flask application entry point for the svg_upload_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /avatar accepts an SVG file upload for user avatar images. Because
SVG is XML, the parser in handler.py processes it as a full XML document.
An attacker can embed an external entity in the SVG to read server files
such as /etc/passwd and have the contents reflected back in the response.

Chain: SVG upload -> ET.parse (entities enabled) -> local file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
from flask import Flask
from handler import svg_bp
import config

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.UPLOAD_FOLDER
app.config["MAX_CONTENT_LENGTH"] = config.MAX_CONTENT_LENGTH
os.makedirs(config.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(svg_bp)

if __name__ == "__main__":
    app.run(port=5000)
