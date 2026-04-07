import csv
import io

def parse_import_csv(raw_text):
    reader = csv.DictReader(io.StringIO(raw_text))
    return [dict(row) for row in reader]
