"""Flask application entry point for the xml_rpc_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /xmlrpc accepts an XML-RPC methodCall document. XML-RPC uses XML
to encode remote procedure calls. The handler in handler.py parses the
body with the default ElementTree parser. An attacker can send a
methodCall containing an external entity pointing to /etc/app/config.ini
and have the configuration file contents returned as a method parameter.

Chain: XML-RPC body -> ET.fromstring (entities enabled) -> config file read
CWE-611: Improper Restriction of XML External Entity Reference
"""
from flask import Flask
from handler import rpc_bp
import config

app = Flask(__name__)
app.register_blueprint(rpc_bp)

if __name__ == "__main__":
    app.run(port=5000)
