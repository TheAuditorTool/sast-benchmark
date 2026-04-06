"""Bulk import row parser -- VULNERABLE variant.

Parses each CSV row into a user dict without sanitising the 'role' column,
so an uploaded CSV can include role=admin to create privileged accounts.

CWE-915: Improperly Controlled Modification of Dynamically-Determined Object Attributes
Chain: POST /import CSV with role=admin -> admin accounts created in bulk
"""
import csv
import io


# vuln-code-snippet start chain_bulk_import_vuln
def parse_import_csv(raw_text):
    """Parse CSV text and return a list of user dicts.

    VULNERABLE: the 'role' column is copied verbatim from the CSV data,
    allowing a crafted upload to create arbitrarily-privileged accounts.
    """
    reader = csv.DictReader(io.StringIO(raw_text))
    return [dict(row) for row in reader]  # vuln-code-snippet vuln-line chain_bulk_import_vuln
# vuln-code-snippet end chain_bulk_import_vuln
