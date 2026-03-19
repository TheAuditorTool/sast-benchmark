package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00113(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("link")
	t, err := template.New("page").Parse(`<html><body><a href="{{.Link}}">Click here</a></body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := struct {
		Link template.URL
	}{
		Link: template.URL(userInput),
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, data)
}
