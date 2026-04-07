package testcode

import (
	"html/template"
	"net/http"
)

var benchmarkTest00718Template = template.Must(template.New("result").Parse(`<html><body><p>Result: {{.Result}}</p></body></html>`))

func BenchmarkTest00718(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("result")
	data := struct{ Result string }{Result: input}
	w.Header().Set("Content-Type", "text/html")
	benchmarkTest00718Template.Execute(w, data)
}
