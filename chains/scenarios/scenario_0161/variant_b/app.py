from flask import Flask
from handler import saml_bp
import config

app = Flask(__name__)
app.secret_key = config.SESSION_SECRET
app.register_blueprint(saml_bp)

if __name__ == "__main__":
    app.run(port=5000)
