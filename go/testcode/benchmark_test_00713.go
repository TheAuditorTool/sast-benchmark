package testcode

import (
	"html/template"
	"net/http"
)

var benchmarkTest00713Template = template.Must(template.New("page").Parse(`<html><body><p>{{.UserInput}}</p></body></html>`))

func BenchmarkTest00713(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("q")
	data := struct{ UserInput string }{UserInput: input}
	w.Header().Set("Content-Type", "text/html")
	benchmarkTest00713Template.Execute(w, data)
}
