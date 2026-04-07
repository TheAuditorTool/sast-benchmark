import re
import subprocess
from flask import Blueprint, request, jsonify

processor_bp = Blueprint("processor", __name__)

_CMD_FORMULA = re.compile(r'^=CMD\((.+)\)$', re.DOTALL)
_MATH_FORMULA = re.compile(r'^[=+\-@]')

_imported_rows: list[list[str]] = []

def evaluate_cell(cell: str) -> str:
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
    return [[evaluate_cell(cell) for cell in row] for row in rows]

@processor_bp.route("/data/process", methods=["POST"])
def process_data():
    if not _imported_rows:
        return jsonify({"error": "No imported data to process"}), 400
    processed = process_rows(_imported_rows)
    return jsonify({"rows": processed, "count": len(processed)}), 200

def store_rows(rows: list[list[str]]) -> None:
    _imported_rows.clear()
    _imported_rows.extend(rows)
