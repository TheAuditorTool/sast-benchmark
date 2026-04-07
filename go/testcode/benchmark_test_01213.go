package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01213(w http.ResponseWriter, r *http.Request) {
	src := r.Header.Get("X-Custom-Template")
	if src == "" {
		src = "<p>Hello</p>"
	}
	t, err := text_template.New("").Parse(src)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	t.Execute(w, map[string]string{"RemoteAddr": r.RemoteAddr})
}
