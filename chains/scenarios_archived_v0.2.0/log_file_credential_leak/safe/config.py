"""Application configuration for the log file credential leak scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

APP_SECRET = os.environ.get("APP_SECRET", "app-secret-do-not-log")
DB_PASSWORD = os.environ.get("DB_PASSWORD", "db-p@ssw0rd-do-not-log")
LOG_FILE = "/tmp/app.log"
