from flask import Flask
from handler import dav_bp
import config

app = Flask(__name__)
app.register_blueprint(dav_bp)

if __name__ == "__main__":
    app.run(port=5000)
