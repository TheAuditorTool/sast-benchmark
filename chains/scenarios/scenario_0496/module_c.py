from module_a import VALID_ROLES

def clean_registration(data):
    return {
        "username": str(data.get("username", "")).strip(),
        "email": str(data.get("email", "")).strip(),
        "org": str(data.get("org", "")).strip(),
        "role": "member",
    }
