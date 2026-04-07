package testcode

import (
	"net/http"
	text_template "text/template"
)

func BenchmarkTest01212(w http.ResponseWriter, r *http.Request) {
	key := r.URL.Query().Get("key")
	row := DB.QueryRow("SELECT content FROM templates WHERE key = ?", key)
	var content string
	if err := row.Scan(&content); err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	t, err := text_template.New("").Parse(content)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	data := map[string]string{"User": r.URL.Query().Get("user")}
	t.Execute(w, data)
}
