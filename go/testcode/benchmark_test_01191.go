package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01191Store = sessions.NewCookieStore([]byte("delete-action-session-key"))

func BenchmarkTest01191(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01191Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	if r.FormValue("csrf_token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	postID := r.FormValue("post_id")
	_, err = DB.Exec("DELETE FROM posts WHERE id = ?", postID)
	if err != nil {
		http.Error(w, "delete failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "post deleted"})
}
