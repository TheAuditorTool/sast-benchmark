package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00124(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("message")
	t, err := template.New("page").Parse(`<html><body><div>{{.}}</div></body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, userInput)
}
