package testcode

import (
	"html/template"
	"net/http"
)

func BenchmarkTest00112(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("callback")
	t, err := template.New("page").Parse(`<html><body><script>{{.Code}}</script></body></html>`)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := struct {
		Code template.JS
	}{
		Code: template.JS(userInput),
	}
	w.Header().Set("Content-Type", "text/html")
	t.Execute(w, data)
}
