package testcode

import "net/http"

func BenchmarkTest00341(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	store := &BenchSvcStore{db: DB}
	err := store.FindUser(id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
