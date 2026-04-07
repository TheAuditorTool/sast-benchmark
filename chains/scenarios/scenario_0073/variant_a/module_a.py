from flask import Flask
from module_c import rpc_bp
import module_b

app = Flask(__name__)
app.register_blueprint(rpc_bp)

if __name__ == "__main__":
    app.run(port=5000)
