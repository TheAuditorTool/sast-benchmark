package testcode

import (
	"net/http"
	html_template "html/template"
)

func BenchmarkTest01223(w http.ResponseWriter, r *http.Request) {
	t, err := html_template.ParseGlob("templates/*.tmpl")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := map[string]string{"User": r.URL.Query().Get("user")}
	if err := t.Execute(w, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
