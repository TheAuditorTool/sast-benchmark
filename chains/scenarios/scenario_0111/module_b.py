import csv
import io
from flask import Blueprint, request, jsonify
from module_c import store_rows

importer_bp = Blueprint("importer", __name__)

MAX_ROWS = 10_000
MAX_COLS = 100

_FORMULA_PREFIX_CHARS = frozenset("=+-@")

def sanitize_cell(cell: str) -> str:
    while cell and cell[0] in _FORMULA_PREFIX_CHARS:
        cell = cell[1:]
    return cell

def parse_csv(data: str) -> list[list[str]]:
    reader = csv.reader(io.StringIO(data))
    rows = []
    for row in reader:
        if len(rows) >= MAX_ROWS:
            break
        rows.append([sanitize_cell(cell) for cell in row[:MAX_COLS]])
    return rows

@importer_bp.route("/data/import", methods=["POST"])
def import_csv():
    content_type = request.content_type or ""
    if "text/csv" not in content_type and "text/plain" not in content_type:
        return jsonify({"error": "Content-Type must be text/csv"}), 415

    raw = request.get_data(as_text=True)
    if not raw:
        return jsonify({"error": "Empty request body"}), 400

    rows = parse_csv(raw)
    store_rows(rows)

    return jsonify({
        "status": "imported",
        "rows": len(rows),
        "process_url": "/data/process",
    }), 201
