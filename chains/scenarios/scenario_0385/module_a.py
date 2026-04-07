import os
from flask import Flask
from module_b import config_bp
import module_c as sec

app = Flask(__name__)
app.register_blueprint(config_bp)

if __name__ == "__main__":
    app.run(port=5000)
