"""Application configuration with database credentials.

This module holds database connection settings. These values are
accessible from within the process via standard attribute access.
An SSTI payload that reaches the Jinja2 engine can traverse the
Python object graph to read these values and exfiltrate them.

This file is IDENTICAL between vuln/ and safe/ variants.
"""

DB_HOST = "db.internal.example.com"
DB_PORT = 5432
DB_NAME = "production"
DB_USER = "app_service"
DB_PASSWORD = "s3cr3t-db-p@ssw0rd"

DATABASE_URL = "postgresql://{user}:{password}@{host}:{port}/{name}".format(
    user=DB_USER,
    password=DB_PASSWORD,
    host=DB_HOST,
    port=DB_PORT,
    name=DB_NAME,
)
