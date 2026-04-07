package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01203(w http.ResponseWriter, r *http.Request) {
	src := r.URL.Query().Get("tmpl")
	t, err := text_template.New("t").Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	data := map[string]string{"User": r.URL.Query().Get("user")}
	if err := t.Execute(w, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}
