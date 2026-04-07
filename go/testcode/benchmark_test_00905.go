package testcode

import (
	"crypto/tls"
	"net/http"

	"github.com/gorilla/websocket"
)

func BenchmarkTest00905(w http.ResponseWriter, r *http.Request) {
	target := r.URL.Query().Get("ws_url")
	dialer := websocket.Dialer{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
	conn, _, err := dialer.Dial(target, nil)
	if err != nil {
		http.Error(w, "ws error", http.StatusBadGateway)
		return
	}
	defer conn.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "connected"})
}
