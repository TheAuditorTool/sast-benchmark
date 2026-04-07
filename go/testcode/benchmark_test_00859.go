package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest00859Store = sessions.NewCookieStore([]byte("session-secret-key"))

func init() {
	benchmarkTest00859Store.Options = &sessions.Options{
		Path:     "/",
		MaxAge:   3600,
		HttpOnly: true,
		Secure:   true,
		SameSite: 3,
	}
}

func BenchmarkTest00859(w http.ResponseWriter, r *http.Request) {
	sess, _ := benchmarkTest00859Store.Get(r, "session")
	sess.Values["user"] = r.FormValue("username")
	benchmarkTest00859Store.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
