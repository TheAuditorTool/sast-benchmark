from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

def send_notification():
    template_url = request.args.get("template_url", "")
    recipient = request.args.get("recipient", "")
    if not template_url or not recipient:
        return jsonify({"error": "template_url and recipient parameters required"}), 400
    
    try:
        response = urllib.request.urlopen(template_url)
        template_body = response.read(65536)
        
        return jsonify({"status": "queued", "recipient": recipient, "template_size": len(template_body)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/notify", view_func=send_notification)

if __name__ == "__main__":
    app.run(port=5016)
