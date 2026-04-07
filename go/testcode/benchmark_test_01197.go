package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01197Store = sessions.NewCookieStore([]byte("upload-meta-session-key-32"))

func BenchmarkTest01197(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01197Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	if r.Header.Get("X-CSRF-Token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	err = r.ParseMultipartForm(10 << 20)
	if err != nil {
		http.Error(w, "parse error", http.StatusBadRequest)
		return
	}
	_, header, err := r.FormFile("file")
	if err != nil {
		http.Error(w, "missing file", http.StatusBadRequest)
		return
	}
	userID := r.FormValue("user_id")
	_, err = DB.Exec("INSERT INTO uploads (user_id, filename) VALUES (?, ?)", userID, header.Filename)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "uploaded"})
}
