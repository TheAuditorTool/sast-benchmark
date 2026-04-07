package testcode

import "net/http"

func BenchmarkTest01067(w http.ResponseWriter, r *http.Request) {
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["subscription_tier"] = r.URL.Query().Get("tier")
	sess.Values["features"] = r.URL.Query().Get("features")
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
