from config import app, LOG_FILE

def log_request(path, body):
    with open(LOG_FILE, "a") as f:
        f.write(path + "\n")

# vuln-code-snippet start ChainScenario0130B
LOG_ENDPOINT_REMOVED = True  # vuln-code-snippet target-line ChainScenario0130B
# vuln-code-snippet end ChainScenario0130B
