package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01207(w http.ResponseWriter, r *http.Request) {
	src := r.FormValue("template")
	t, err := text_template.New("").Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	data := map[string]string{"Name": r.FormValue("name")}
	t.Execute(w, data)
}
