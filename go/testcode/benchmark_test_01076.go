package testcode

import (
	"net/http"

	"github.com/gorilla/securecookie"
	"github.com/gorilla/sessions"
)

var benchmarkTest01076Store = sessions.NewCookieStore(
	securecookie.GenerateRandomKey(64),
	securecookie.GenerateRandomKey(32),
)

func BenchmarkTest01076(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	var userID int
	err := DB.QueryRow("SELECT id FROM users WHERE username = ? AND password_hash = ?", username, password).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	sess, _ := benchmarkTest01076Store.Get(r, "session")
	sess.Values["user_id"] = userID
	benchmarkTest01076Store.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
