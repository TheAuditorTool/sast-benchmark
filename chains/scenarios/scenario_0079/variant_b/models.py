from dataclasses import dataclass, field
from typing import List, Dict, Any

@dataclass
class UserData:
    username: str
    email: str
    records: List[Dict[str, Any]] = field(default_factory=list)

    def summary(self):
        return (
            f"UserData(username={self.username!r}, "
            f"email={self.email!r}, "
            f"records={len(self.records)})"
        )

    def validate(self):
        if not self.username or len(self.username) < 1:
            raise ValueError("username is required")
        if not self.email or "@" not in self.email:
            raise ValueError("valid email is required")
        return True
