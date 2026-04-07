"""Configuration for the pom_xml_xxe scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os

CI_SECRET_FILE = "/run/secrets/ci_token"
NEXUS_PASSWORD = os.environ.get("NEXUS_PASSWORD", "nexus-pass-placeholder")
BUILD_SERVICE_TOKEN = os.environ.get("BUILD_SERVICE_TOKEN", "build-token-placeholder")
MAX_POM_BYTES = 256 * 1024
