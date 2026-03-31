"""Data models for user import/export.

This file is IDENTICAL between vuln/ and safe/ variants.
The UserData class represents a legitimate serializable record
used by the import/export endpoints.
"""
from dataclasses import dataclass, field
from typing import List, Dict, Any


@dataclass
class UserData:
    """Represents a user's exported data bundle."""
    username: str
    email: str
    records: List[Dict[str, Any]] = field(default_factory=list)

    def summary(self):
        """Return a human-readable summary of this data bundle."""
        return (
            f"UserData(username={self.username!r}, "
            f"email={self.email!r}, "
            f"records={len(self.records)})"
        )

    def validate(self):
        """Basic validation of required fields."""
        if not self.username or len(self.username) < 1:
            raise ValueError("username is required")
        if not self.email or "@" not in self.email:
            raise ValueError("valid email is required")
        return True
