from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0166A
DEBUG_SESSIONS_REMOVED = True  # vuln-code-snippet target-line ChainScenario0166A
# vuln-code-snippet end ChainScenario0166A
