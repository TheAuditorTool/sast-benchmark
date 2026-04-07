from flask import Flask
from module_c import dav_bp
import module_b

app = Flask(__name__)
app.register_blueprint(dav_bp)

if __name__ == "__main__":
    app.run(port=5000)
