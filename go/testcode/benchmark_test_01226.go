package testcode

import (
	html_template "html/template"
	"net/http"
	"strings"
)

var benchmarkTest01226Tmpl = html_template.Must(html_template.New("t").Parse("<p>{{.}}</p>"))

func BenchmarkTest01226(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("data")
	safe := strings.ReplaceAll(strings.ReplaceAll(input, "{{", ""), "}}", "")
	benchmarkTest01226Tmpl.Execute(w, safe)
}
