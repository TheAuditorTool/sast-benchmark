package testcode

import (
	"crypto/tls"
	"net/http"

	"github.com/gorilla/websocket"
)

func BenchmarkTest00653(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL string `json:"url"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	dialer := websocket.Dialer{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}

	conn, _, err := dialer.Dial(req.URL, nil)
	if err != nil {
		http.Error(w, "websocket dial failed", http.StatusBadGateway)
		return
	}
	defer conn.Close()

	if err := conn.WriteMessage(websocket.TextMessage, []byte("ping")); err != nil {
		http.Error(w, "write failed", http.StatusInternalServerError)
		return
	}

	_, msg, err := conn.ReadMessage()
	if err != nil {
		http.Error(w, "read failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"response": string(msg)})
}
