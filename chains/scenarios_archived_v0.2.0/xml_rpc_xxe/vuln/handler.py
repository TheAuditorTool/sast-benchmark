"""XML-RPC method call handler -- VULNERABLE variant.

POST /xmlrpc parses an XML-RPC methodCall body with the default
xml.etree.ElementTree parser. External entities are resolved before
the method name and parameters are extracted. An attacker can read
/etc/app/config.ini via:

    <!DOCTYPE methodCall [<!ENTITY cfg SYSTEM "file:///etc/app/config.ini">]>
    <methodCall>
      <methodName>system.getInfo</methodName>
      <params><param><value><string>&cfg;</string></value></param></params>
    </methodCall>

The config file contents appear as the first parameter value in the
extracted call, which is then returned in the JSON response.

Chain: XML-RPC body -> ET.fromstring (entities enabled) -> config file read
Individual findings: XXE in XML-RPC endpoint (high)
Chain finding: XXE enables reading application config via XML-RPC (critical)
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


@rpc_bp.route("/xmlrpc", methods=["POST"])
def xmlrpc():
    """Parse an XML-RPC method call and return the method name and params."""
    raw = request.get_data()
    if len(raw) > config.XMLRPC_MAX_BYTES:
        return jsonify({"error": "request too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start chain_xmlrpc_xxe_vuln
        root = ET.fromstring(xml_text)  # vuln-code-snippet vuln-line chain_xmlrpc_xxe_vuln
# vuln-code-snippet end chain_xmlrpc_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 400
    method, params = _extract_call(root)
    if not method:
        return jsonify({"error": "missing methodName"}), 400
    return jsonify({"method": method, "params": params})
