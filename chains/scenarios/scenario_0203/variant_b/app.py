from flask import Flask
from handler import sitemap_bp
import config

app = Flask(__name__)
app.register_blueprint(sitemap_bp)

if __name__ == "__main__":
    app.run(port=5000)
