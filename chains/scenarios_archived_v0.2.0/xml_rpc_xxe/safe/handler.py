"""XML-RPC method call handler -- SAFE variant.

POST /xmlrpc parses an XML-RPC methodCall body with a hardened
XMLParser. Clearing parser.entity prevents external entity declarations
from being resolved, so crafted calls targeting /etc/app/config.ini
or other paths produce no file read.

Chain broken: XMLParser with entity={} prevents entity resolution ->
application config read via XML-RPC is prevented
CWE-611: Improper Restriction of XML External Entity Reference
"""
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

rpc_bp = Blueprint("rpc", __name__)


def _extract_call(root):
    method = root.findtext("methodName") or ""
    params = []
    for param in root.findall("params/param"):
        val_el = param.find("value")
        if val_el is not None:
            text = "".join(val_el.itertext())
            params.append(text)
    return method, params


def _safe_fromstring(text):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(text, parser=parser)


@rpc_bp.route("/xmlrpc", methods=["POST"])
def xmlrpc():
    """Parse an XML-RPC method call and return the method name and params."""
    raw = request.get_data()
    if len(raw) > config.XMLRPC_MAX_BYTES:
        return jsonify({"error": "request too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_xmlrpc_xxe_safe
        root = _safe_fromstring(xml_text)  # vuln-code-snippet safe-line chain_xmlrpc_xxe_safe
# vuln-code-snippet end chain_xmlrpc_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 400
    method, params = _extract_call(root)
    if not method:
        return jsonify({"error": "missing methodName"}), 400
    return jsonify({"method": method, "params": params})
