from flask import Flask
from handler import rpc_bp
import config

app = Flask(__name__)
app.register_blueprint(rpc_bp)

if __name__ == "__main__":
    app.run(port=5000)
