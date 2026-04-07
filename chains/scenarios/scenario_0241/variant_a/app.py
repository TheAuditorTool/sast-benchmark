import os
from flask import Flask
from parser import config_bp
import secrets as sec

app = Flask(__name__)
app.register_blueprint(config_bp)

if __name__ == "__main__":
    app.run(port=5000)
