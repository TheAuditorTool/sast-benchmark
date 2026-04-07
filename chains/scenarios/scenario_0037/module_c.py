import csv
import io

_ALLOWED_COLUMNS = {"username", "email"}

def parse_import_csv(raw_text):
    reader = csv.DictReader(io.StringIO(raw_text))
    return [{k: v for k, v in row.items() if k in _ALLOWED_COLUMNS} for row in reader]
