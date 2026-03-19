package testcode

import (
	"net/http"

	"github.com/gorilla/websocket"
)

var WsUpgrader = websocket.Upgrader{CheckOrigin: func(r *http.Request) bool { return true }}

func BenchmarkTest00417(w http.ResponseWriter, r *http.Request) {
	conn, err := WsUpgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()
	for {
		mt, message, err := conn.ReadMessage()
		if err != nil {
			break
		}
		conn.WriteMessage(mt, message)
	}
}
