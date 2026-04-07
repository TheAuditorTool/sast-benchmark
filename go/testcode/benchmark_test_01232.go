package testcode

import (
	"net/http"
	text_template "text/template"
)

var benchmarkTest01232Tmpl = text_template.Must(text_template.New("t").Parse("<p>{{.}}</p>"))

func BenchmarkTest01232(w http.ResponseWriter, r *http.Request) {
	raw := r.URL.Query().Get("data")
	escaped := text_template.HTMLEscapeString(raw)
	benchmarkTest01232Tmpl.Execute(w, escaped)
}
