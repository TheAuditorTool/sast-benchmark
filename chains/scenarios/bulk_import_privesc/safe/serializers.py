"""Bulk import row parser -- SAFE variant.

Parses each CSV row but discards the 'role' column entirely.  All
imported accounts receive the default role regardless of CSV content.

CWE-915: Fixed by ignoring the 'role' column during CSV parsing.
Chain: POST /import CSV with role=admin -> role column discarded -> accounts created as 'user'
"""
import csv
import io

_ALLOWED_COLUMNS = {"username", "email"}


# vuln-code-snippet start chain_bulk_import_safe
def parse_import_csv(raw_text):
    """Parse CSV text and return user dicts limited to allowed columns.

    SAFE: the 'role' column is excluded; callers cannot escalate privileges
    by crafting CSV content.
    """
    reader = csv.DictReader(io.StringIO(raw_text))
    return [{k: v for k, v in row.items() if k in _ALLOWED_COLUMNS} for row in reader]  # vuln-code-snippet safe-line chain_bulk_import_safe
# vuln-code-snippet end chain_bulk_import_safe
