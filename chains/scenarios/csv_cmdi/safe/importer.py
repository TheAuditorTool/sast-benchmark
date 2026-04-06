"""CSV importer -- SAFE variant.

Accepts an uploaded CSV file, parses it, and sanitizes all cells by stripping
leading formula-prefix characters (=, +, -, @) before storing. This prevents
processor.py's CMD evaluator from treating any user-supplied value as a formula,
regardless of what the CSV contains.

Chain: POST /data/import with CSV containing =CMD(id) -> sanitize_cell() strips
       leading = -> cell stored as CMD(id) (plain string) -> POST /data/process ->
       evaluate_cell() sees no formula prefix -> cell returned unchanged -> no RCE
Individual findings: none -- formula prefixes stripped during import
Chain finding: none -- CSV formula injection prevented at parse step (CWE-78 mitigated)
"""
import csv
import io
from flask import Blueprint, request, jsonify
from processor import store_rows

importer_bp = Blueprint("importer", __name__)

MAX_ROWS = 10_000
MAX_COLS = 100

_FORMULA_PREFIX_CHARS = frozenset("=+-@")


def sanitize_cell(cell: str) -> str:
    """Strip leading formula-prefix characters from a CSV cell value.

    Prevents spreadsheet formula injection (DDE attacks) by ensuring cells
    beginning with =, +, -, or @ are treated as plain text.
    """
    while cell and cell[0] in _FORMULA_PREFIX_CHARS:
        cell = cell[1:]
    return cell


def parse_csv(data: str) -> list[list[str]]:
    """Parse CSV text into a list of sanitized rows."""
    reader = csv.reader(io.StringIO(data))
    rows = []
    for row in reader:
        if len(rows) >= MAX_ROWS:
            break
        rows.append([sanitize_cell(cell) for cell in row[:MAX_COLS]])
    return rows


# vuln-code-snippet start chain_csv_cmdi_safe
@importer_bp.route("/data/import", methods=["POST"])
def import_csv():
    """Import a CSV data file.

    Accepts text/csv content in the request body. Formula-prefix characters
    are stripped from all cells before storage.
    """
    content_type = request.content_type or ""
    if "text/csv" not in content_type and "text/plain" not in content_type:
        return jsonify({"error": "Content-Type must be text/csv"}), 415

    raw = request.get_data(as_text=True)
    if not raw:
        return jsonify({"error": "Empty request body"}), 400

    rows = parse_csv(raw)  # vuln-code-snippet safe-line chain_csv_cmdi_safe
    store_rows(rows)

    return jsonify({
        "status": "imported",
        "rows": len(rows),
        "process_url": "/data/process",
    }), 201
# vuln-code-snippet end chain_csv_cmdi_safe
