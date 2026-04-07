package testcode

import "net/http"

func BenchmarkTest01057(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["tenant"] = r.FormValue("tenant")
	sess.Values["user"] = r.FormValue("username")
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
