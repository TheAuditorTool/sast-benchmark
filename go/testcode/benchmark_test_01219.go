package testcode

import (
	"embed"
	"net/http"
	html_template "html/template"
)

//go:embed templates
var benchmarkTest01219FS embed.FS

func BenchmarkTest01219(w http.ResponseWriter, r *http.Request) {
	t, err := html_template.ParseFS(benchmarkTest01219FS, "templates/*.tmpl")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := map[string]string{"User": r.URL.Query().Get("user")}
	if err := t.Execute(w, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
