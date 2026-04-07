import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
import config

pom_bp = Blueprint("pom", __name__)

_MVN_NS = "http://maven.apache.org/POM/4.0.0"

@pom_bp.route("/build/analyze", methods=["POST"])
def analyze_pom():
    raw = request.get_data()
    if len(raw) > config.MAX_POM_BYTES:
        return jsonify({"error": "POM too large"}), 413
    xml_text = raw.decode("utf-8", errors="replace")
    try:
# vuln-code-snippet start ChainScenario0040A
        root = ET.fromstring(xml_text)  # vuln-code-snippet target-line ChainScenario0040A
# vuln-code-snippet end ChainScenario0040A
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    deps = []
    for dep in root.iter("dependency"):
        g = dep.findtext("groupId") or dep.findtext(f"{{{_MVN_NS}}}groupId") or ""
        a = dep.findtext("artifactId") or dep.findtext(f"{{{_MVN_NS}}}artifactId") or ""
        v = dep.findtext("version") or dep.findtext(f"{{{_MVN_NS}}}version") or ""
        deps.append({"groupId": g, "artifactId": a, "version": v})
    return jsonify({"dependencies": deps})
