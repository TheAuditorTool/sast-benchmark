package testcode

import (
	"log"
	"net/http"
)

type benchmarkTest00969Logger struct{}

func (l benchmarkTest00969Logger) Log(msg string) {
	log.Println(msg)
}

func BenchmarkTest00969(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("message")
	logger := benchmarkTest00969Logger{}
	logger.Log("user: " + userInput)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
