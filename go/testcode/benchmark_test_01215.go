package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01215(w http.ResponseWriter, r *http.Request) {
	pattern := r.URL.Query().Get("pattern")
	t, err := text_template.ParseGlob(pattern)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if err := t.Execute(w, nil); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
