from flask import Flask
from extractor import extractor_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(extractor_bp)

if __name__ == "__main__":
    app.run(port=5000)
