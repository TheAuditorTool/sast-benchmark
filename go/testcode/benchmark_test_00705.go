package testcode

import (
	"net/http"
	"text/template"
)

var benchmarkTest00705Template = template.Must(template.New("page").Parse(`<html><body><div>{{.Content}}</div></body></html>`))

func BenchmarkTest00705(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("content")
	data := struct{ Content string }{Content: userInput}
	w.Header().Set("Content-Type", "text/html")
	benchmarkTest00705Template.Execute(w, data)
}
