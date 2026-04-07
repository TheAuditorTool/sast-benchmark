from module_b import app, LOG_FILE

def log_request(path, body):
    with open(LOG_FILE, "a") as f:
        f.write(path + "\n")

LOG_ENDPOINT_REMOVED = True
