from flask import Flask

app = Flask(__name__)

from views import page_bp

app.register_blueprint(page_bp)

if __name__ == "__main__":
    app.run(port=5000)
