from flask import Flask

app = Flask(__name__)

from auth import mail_bp
from routes import routes_bp

app.register_blueprint(mail_bp)
app.register_blueprint(routes_bp)

if __name__ == "__main__":
    app.run(port=5000)
