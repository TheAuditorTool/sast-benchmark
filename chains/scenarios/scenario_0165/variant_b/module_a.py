import os
from flask import Flask
from module_c import svg_bp
import module_b

app = Flask(__name__)
app.config["UPLOAD_FOLDER"] = config.UPLOAD_FOLDER
app.config["MAX_CONTENT_LENGTH"] = config.MAX_CONTENT_LENGTH
os.makedirs(config.UPLOAD_FOLDER, exist_ok=True)

app.register_blueprint(svg_bp)

if __name__ == "__main__":
    app.run(port=5000)
