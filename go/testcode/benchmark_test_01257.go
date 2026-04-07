package testcode

import (
	"net/http"
)

type benchmarkTest01257Payload struct {
	User  string
	Score int
}

func BenchmarkTest01257(w http.ResponseWriter, r *http.Request) {
	p := benchmarkTest01257Payload{
		User:  r.URL.Query().Get("user"),
		Score: 100,
	}
	ch := make(chan benchmarkTest01257Payload, 1)
	ch <- p
	go func() {
		msg := <-ch
		DB.Exec("INSERT INTO scores (user, score) VALUES (?, ?)", msg.User, msg.Score)
	}()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "queued"})
}
