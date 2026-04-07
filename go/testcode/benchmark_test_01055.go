package testcode

import "net/http"

func BenchmarkTest01055(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["email"] = r.Header.Get("X-User-Email")
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
