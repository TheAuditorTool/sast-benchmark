REGISTERED_URIS = {
    "client_app_1": {"https://app.example.com/callback"},
    "client_app_2": {"https://partner.example.com/oauth/callback"},
}

def validate_redirect_uri(redirect_uri, client_id):
    allowed = REGISTERED_URIS.get(client_id, set())
    return redirect_uri in allowed
