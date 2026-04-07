import os
from flask import Flask
from parser import xliff_bp
import config

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.TRANSLATION_UPLOAD_FOLDER
os.makedirs(config.TRANSLATION_UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(xliff_bp)

if __name__ == "__main__":
    app.run(port=5000)
