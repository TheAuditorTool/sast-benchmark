from flask import Flask
from module_c import feed_bp
import module_b
import os

app = Flask(__name__)
os.makedirs(config.FEED_CACHE_DIR, exist_ok=True)
app.register_blueprint(feed_bp)

if __name__ == "__main__":
    app.run(port=5000)
