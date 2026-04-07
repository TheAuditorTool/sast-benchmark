from flask import Flask
from module_c import reg_bp
from module_b import reset_bp

app = Flask(__name__)
app.register_blueprint(reg_bp)
app.register_blueprint(reset_bp)

import module_c
import module_b
password_reset.USER_STORE = registration.USER_STORE

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
