from flask import Flask
from parser import feed_bp
import config
import os

app = Flask(__name__)
os.makedirs(config.FEED_CACHE_DIR, exist_ok=True)
app.register_blueprint(feed_bp)

if __name__ == "__main__":
    app.run(port=5000)
