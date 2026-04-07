from flask import Flask, request, jsonify
import urllib.request

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0005B
def receive_webhook():
    data = request.get_json()
    callback_url = data.get("callback_url", "")
    
    response = urllib.request.urlopen(callback_url)  # vuln-code-snippet target-line ChainScenario0005B
    return jsonify({"status": "received", "verified": response.status == 200})
# vuln-code-snippet end ChainScenario0005B

app.add_url_rule("/api/webhook", view_func=receive_webhook, methods=["POST"])
