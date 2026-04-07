import csv
import io

_ALLOWED_COLUMNS = {"username", "email"}

# vuln-code-snippet start ChainScenario0123B
def parse_import_csv(raw_text):
    reader = csv.DictReader(io.StringIO(raw_text))
    return [{k: v for k, v in row.items() if k in _ALLOWED_COLUMNS} for row in reader]  # vuln-code-snippet target-line ChainScenario0123B
# vuln-code-snippet end ChainScenario0123B
