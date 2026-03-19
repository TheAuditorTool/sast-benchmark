package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00111(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("name")
	t, err := template.New("page").Parse(`<html><body>{{.Content}}</body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := struct {
		Content template.HTML
	}{
		Content: template.HTML(userInput),
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, data)
}
