package testcode

import (
	"html"
	"net/http"
)

func BenchmarkTest00418(w http.ResponseWriter, r *http.Request) {
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
		escaped := html.EscapeString(string(message))
		conn.WriteMessage(mt, []byte(escaped))
	}
}
