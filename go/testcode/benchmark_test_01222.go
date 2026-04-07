package testcode

import (
	"embed"
	html_template "html/template"
	"net/http"
)

//go:embed templates
var benchmarkTest01222FS embed.FS

var benchmarkTest01222Tmpl = html_template.Must(html_template.ParseFS(benchmarkTest01222FS, "templates/*.tmpl"))

func BenchmarkTest01222(w http.ResponseWriter, r *http.Request) {
	data := map[string]string{"Name": r.URL.Query().Get("name")}
	if err := benchmarkTest01222Tmpl.Execute(w, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
