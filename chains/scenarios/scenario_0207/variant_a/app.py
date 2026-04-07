from flask import Flask
from registration import reg_bp
from password_reset import reset_bp

app = Flask(__name__)
app.register_blueprint(reg_bp)
app.register_blueprint(reset_bp)

import registration
import password_reset
password_reset.USER_STORE = registration.USER_STORE

@app.route("/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    app.run(port=5000)
