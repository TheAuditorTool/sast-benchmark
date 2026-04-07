package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

func benchmarkTest01059PopulateSession(sess *sessions.Session, r *http.Request) {
	for key, vals := range r.Form {
		if len(vals) > 0 {
			sess.Values[key] = vals[0]
		}
	}
}

func BenchmarkTest01059(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	benchmarkTest01059PopulateSession(sess, r)
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
