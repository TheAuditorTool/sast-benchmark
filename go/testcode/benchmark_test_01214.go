package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01214(w http.ResponseWriter, r *http.Request) {
	src := r.URL.Query().Get("tmpl")
	t, err := text_template.New("").Delims("[[", "]]").Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	data := map[string]string{"Value": r.URL.Query().Get("val")}
	t.Execute(w, data)
}
