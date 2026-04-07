package testcode

import (
	html_template "html/template"
	"net/http"
	"net/url"
)

var benchmarkTest01230Tmpl = html_template.Must(html_template.New("t").Parse(`<a href="/view/{{.Segment}}">View</a>`))

func BenchmarkTest01230(w http.ResponseWriter, r *http.Request) {
	raw := r.URL.Query().Get("path")
	escaped := url.QueryEscape(raw)
	data := struct{ Segment string }{Segment: escaped}
	benchmarkTest01230Tmpl.Execute(w, data)
}
