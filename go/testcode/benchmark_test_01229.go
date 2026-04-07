package testcode

import (
	html_template "html/template"
	"net/http"
)

var benchmarkTest01229Tmpl = html_template.Must(html_template.New("t").Parse(`<script>var x={{.}};</script>`))

func BenchmarkTest01229(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("val")
	benchmarkTest01229Tmpl.Execute(w, userInput)
}
