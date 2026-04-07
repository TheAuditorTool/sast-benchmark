package testcode

import (
	"net/http"
	text_template "text/template"
)

var benchmarkTest01228Tmpl = text_template.Must(text_template.New("t").Parse(`{{/* user comment not rendered */}}<p>Hello, {{.Name}}!</p>`))

func BenchmarkTest01228(w http.ResponseWriter, r *http.Request) {
	data := struct{ Name string }{Name: r.URL.Query().Get("name")}
	benchmarkTest01228Tmpl.Execute(w, data)
}
