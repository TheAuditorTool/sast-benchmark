package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01217(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("tmpl")
	if err != nil {
		http.Error(w, "missing cookie", http.StatusBadRequest)
		return
	}
	t, err := text_template.New("").Parse(cookie.Value)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	data := map[string]string{"Path": r.URL.Path}
	t.Execute(w, data)
}
