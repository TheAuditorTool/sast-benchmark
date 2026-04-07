"""CSV cell processor -- IDENTICAL between vuln/ and safe/ variants.

Evaluates cells that begin with a formula-prefix character (=, +, -, @) to
support computed fields in uploaded data. The =CMD(...) pseudo-formula is
specifically handled by executing the argument as a shell command and returning
its output, mimicking the DDE (Dynamic Data Exchange) attack surface present
in spreadsheet applications.

This is the RCE sink in the chain. It is safe when importer.py strips formula
prefixes before passing data here, but executes attacker commands when the CSV
is imported without sanitization.

Chain: importer.py passes raw cell containing =CMD(id) -> evaluate_cell() matches
       CMD prefix -> subprocess.run executes `id` -> output returned in response
Individual findings: shell execution of formula values (high, dangerous by design)
Chain finding: combined with unsanitized CSV import, enables OS command execution
               (critical, CWE-78)
"""
import re
import subprocess
from flask import Blueprint, request, jsonify

processor_bp = Blueprint("processor", __name__)

_CMD_FORMULA = re.compile(r'^=CMD\((.+)\)$', re.DOTALL)
_MATH_FORMULA = re.compile(r'^[=+\-@]')

_imported_rows: list[list[str]] = []


def evaluate_cell(cell: str) -> str:
    """Evaluate a single CSV cell value.

    Cells starting with =CMD(...) execute the enclosed string as a shell command.
    Other formula-prefix cells are returned unchanged (formula evaluation is
    out of scope for this demo).
    """
    cmd_match = _CMD_FORMULA.match(cell)
    if cmd_match:
        command = cmd_match.group(1)
        result = subprocess.run(
            command,
            shell=True,
            capture_output=True,
            text=True,
            timeout=10,
        )
        return result.stdout.strip() or result.stderr.strip()
    if _MATH_FORMULA.match(cell):
        return f"[formula: {cell}]"
    return cell


def process_rows(rows: list[list[str]]) -> list[list[str]]:
    """Evaluate all cells in a row set, returning the processed result."""
    return [[evaluate_cell(cell) for cell in row] for row in rows]


@processor_bp.route("/data/process", methods=["POST"])
def process_data():
    """Process all currently imported rows through the cell evaluator."""
    if not _imported_rows:
        return jsonify({"error": "No imported data to process"}), 400
    processed = process_rows(_imported_rows)
    return jsonify({"rows": processed, "count": len(processed)}), 200


def store_rows(rows: list[list[str]]) -> None:
    """Store rows for processing (called by importer)."""
    _imported_rows.clear()
    _imported_rows.extend(rows)
