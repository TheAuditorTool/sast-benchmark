package testcode

import (
	"net/http"
	"text/template"
)

func BenchmarkTest00590(w http.ResponseWriter, r *http.Request) {
	pattern := r.FormValue("pattern")
	if pattern == "" {
		http.Error(w, "pattern required", http.StatusBadRequest)
		return
	}

	tmpl := template.Must(template.New("").ParseGlob(pattern))

	w.Header().Set("Content-Type", "text/html")
	if err := tmpl.Execute(w, nil); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
