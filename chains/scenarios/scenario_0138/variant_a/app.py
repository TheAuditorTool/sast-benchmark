import os
from flask import Flask
from parser import parser_bp
import secrets as cfg

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = cfg.UPLOAD_FOLDER
os.makedirs(cfg.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(parser_bp)

if __name__ == "__main__":
    app.run(port=5000)
