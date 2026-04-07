"""CSV importer -- VULNERABLE variant.

Accepts an uploaded CSV file, parses it, and stores the raw rows for later
processing. Cells are stored as-is without stripping formula-prefix characters
(=, +, -, @). When processor.py subsequently evaluates these cells, any cell
containing =CMD(...) will be executed as a shell command.

Chain: POST /data/import with CSV containing =CMD(id) -> parse_csv() returns
       raw cells -> store_rows() stores them -> POST /data/process ->
       evaluate_cell() executes the command -> OS command injection
Individual findings: formula prefixes not stripped from CSV cells (high)
Chain finding: combined with the processor's CMD evaluator, enables OS command
               execution (critical, CWE-78)
"""
import csv
import io
from flask import Blueprint, request, jsonify
from processor import store_rows

importer_bp = Blueprint("importer", __name__)

MAX_ROWS = 10_000
MAX_COLS = 100


def parse_csv(data: str) -> list[list[str]]:
    """Parse CSV text into a list of rows.

    Returns cells as-is without any sanitization.
    """
    reader = csv.reader(io.StringIO(data))
    rows = []
    for row in reader:
        if len(rows) >= MAX_ROWS:
            break
        rows.append(row[:MAX_COLS])
    return rows


# vuln-code-snippet start chain_csv_cmdi_vuln
@importer_bp.route("/data/import", methods=["POST"])
def import_csv():
    """Import a CSV data file.

    Accepts text/csv content in the request body. Cells are stored without
    stripping formula-prefix characters.
    """
    content_type = request.content_type or ""
    if "text/csv" not in content_type and "text/plain" not in content_type:
        return jsonify({"error": "Content-Type must be text/csv"}), 415

    raw = request.get_data(as_text=True)
    if not raw:
        return jsonify({"error": "Empty request body"}), 400

    rows = parse_csv(raw)  # vuln-code-snippet vuln-line chain_csv_cmdi_vuln
    store_rows(rows)

    return jsonify({
        "status": "imported",
        "rows": len(rows),
        "process_url": "/data/process",
    }), 201
# vuln-code-snippet end chain_csv_cmdi_vuln
