package testcode

import (
	"net/http"

	"github.com/gorilla/websocket"
)

func BenchmarkTest00726(w http.ResponseWriter, r *http.Request) {
	wsURL := r.URL.Query().Get("ws_url")
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		http.Error(w, "dial error", http.StatusBadGateway)
		return
	}
	defer conn.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "connected"})
}
