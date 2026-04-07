_ALLOWED_FIELDS = {"username", "password", "email"}

def deserialize_registration(data):
    user = {k: v for k, v in data.items() if k in _ALLOWED_FIELDS}
    user.setdefault("role", "user")
    return user
