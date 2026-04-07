from flask import Flask
from handler import soap_bp
import config

app = Flask(__name__)
app.register_blueprint(soap_bp)

if __name__ == "__main__":
    app.run(port=5000)
