import csv
import io

# vuln-code-snippet start ChainScenario0123A
def parse_import_csv(raw_text):
    reader = csv.DictReader(io.StringIO(raw_text))
    return [dict(row) for row in reader]  # vuln-code-snippet target-line ChainScenario0123A
# vuln-code-snippet end ChainScenario0123A
