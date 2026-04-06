package testcode

import (
	"embed"
	"net/http"
	"text/template"
)

//go:embed templates/greeting.tmpl
var benchmarkTest00595FS embed.FS

type benchmarkTest00595Data struct {
	Username string
	Role     string
}

func BenchmarkTest00595(w http.ResponseWriter, r *http.Request) {
	var data benchmarkTest00595Data
	if err := ParseJSONBody(r, &data); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	tmpl, err := template.ParseFS(benchmarkTest00595FS, "templates/greeting.tmpl")
	if err != nil {
		http.Error(w, "template load error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	if err := tmpl.Execute(w, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
