package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest00866Store = sessions.NewCookieStore([]byte("session-secret-key"))

func BenchmarkTest00866(w http.ResponseWriter, r *http.Request) {
	sess, _ := benchmarkTest00866Store.Get(r, "session")
	sess.Options = &sessions.Options{
		Path:     "/",
		MaxAge:   3600,
		HttpOnly: true,
		Secure:   true,
	}
	sess.Values["user"] = r.FormValue("username")
	benchmarkTest00866Store.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
