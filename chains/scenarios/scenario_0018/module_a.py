from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-key"
app.config["SESSION_COOKIE_SAMESITE"] = "Lax"

import module_c  
