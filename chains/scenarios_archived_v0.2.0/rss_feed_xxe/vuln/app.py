"""Flask application entry point for the rss_feed_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /feed/import accepts a raw RSS 2.0 XML body and delegates to
parser.py to extract feed items. A malicious feed can include an
external entity referencing a local file; the vulnerable parser
resolves it and exposes the file contents inside the extracted items.

Chain: RSS feed body -> ET.fromstring (entities enabled) -> file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from parser import feed_bp
import config
import os

app = Flask(__name__)
os.makedirs(config.FEED_CACHE_DIR, exist_ok=True)
app.register_blueprint(feed_bp)

if __name__ == "__main__":
    app.run(port=5000)
