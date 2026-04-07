package testcode

import (
	"html/template"
	"net/http"
)

var benchmarkTest00703Template = template.Must(template.New("page").Parse(`<html><body>{{.Content}}</body></html>`))

func BenchmarkTest00703(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("content")
	data := struct {
		Content template.HTML
	}{
		Content: template.HTML(userInput),
	}
	w.Header().Set("Content-Type", "text/html")
	benchmarkTest00703Template.Execute(w, data)
}
