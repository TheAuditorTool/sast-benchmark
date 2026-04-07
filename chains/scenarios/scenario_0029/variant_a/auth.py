from models import INVITE_TOKENS

def validate_invite(token):
    return token in INVITE_TOKENS

def consume_invite(token):
    INVITE_TOKENS.discard(token)
