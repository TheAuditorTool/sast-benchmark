import os
from flask import Flask
from parser import docx_bp
import config

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.UPLOAD_FOLDER
os.makedirs(config.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(docx_bp)

if __name__ == "__main__":
    app.run(port=5000)
