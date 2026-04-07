package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest00856Store = sessions.NewCookieStore([]byte("session-secret-key"))

func BenchmarkTest00856(w http.ResponseWriter, r *http.Request) {
	sess, _ := benchmarkTest00856Store.Get(r, "session")
	sess.Options = &sessions.Options{
		Path:     "/",
		MaxAge:   3600,
		HttpOnly: true,
		Secure:   false,
	}
	sess.Values["user"] = r.FormValue("username")
	benchmarkTest00856Store.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
