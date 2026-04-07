import csv
import io
from flask import Blueprint, request, jsonify
from processor import store_rows

importer_bp = Blueprint("importer", __name__)

MAX_ROWS = 10_000
MAX_COLS = 100

def parse_csv(data: str) -> list[list[str]]:
    reader = csv.reader(io.StringIO(data))
    rows = []
    for row in reader:
        if len(rows) >= MAX_ROWS:
            break
        rows.append(row[:MAX_COLS])
    return rows

# vuln-code-snippet start ChainScenario0047A
@importer_bp.route("/data/import", methods=["POST"])
def import_csv():
    content_type = request.content_type or ""
    if "text/csv" not in content_type and "text/plain" not in content_type:
        return jsonify({"error": "Content-Type must be text/csv"}), 415

    raw = request.get_data(as_text=True)
    if not raw:
        return jsonify({"error": "Empty request body"}), 400

    rows = parse_csv(raw)  # vuln-code-snippet target-line ChainScenario0047A
    store_rows(rows)

    return jsonify({
        "status": "imported",
        "rows": len(rows),
        "process_url": "/data/process",
    }), 201
# vuln-code-snippet end ChainScenario0047A
