from flask import Flask
from handler import xslt_bp
import config

app = Flask(__name__)
app.register_blueprint(xslt_bp)

if __name__ == "__main__":
    app.run(port=5000)
