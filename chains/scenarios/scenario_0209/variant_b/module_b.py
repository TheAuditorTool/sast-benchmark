from flask import Flask

app = Flask(__name__)
app.secret_key = "hr-service-secret"

PROFILES = {
    "emp-001": {"name": "Grace", "email": "grace@corp.com", "role": "employee", "department": "Engineering"},
    "emp-002": {"name": "Heidi", "email": "heidi@corp.com", "role": "employee", "department": "HR"},
    "mgr-001": {"name": "Ivan", "email": "ivan@corp.com", "role": "manager", "department": "Engineering"},
}

SELF_EDITABLE_FIELDS = {"name", "email", "department"}
