package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01205(w http.ResponseWriter, r *http.Request) {
	path := r.URL.Query().Get("file")
	t, err := text_template.ParseFiles(path)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if err := t.Execute(w, nil); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
