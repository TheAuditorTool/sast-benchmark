package testcode

import (
	"net/http"

	"github.com/gorilla/csrf"
	"github.com/gorilla/mux"
)

var benchmarkTest01188Key = []byte("32-byte-auth-key-for-csrf-protect")

func benchmarkTest01188Handler(w http.ResponseWriter, r *http.Request) {
	userID := r.FormValue("user_id")
	newEmail := r.FormValue("email")
	_, err := DB.Exec("UPDATE users SET email = ? WHERE id = ?", newEmail, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

func BenchmarkTest01188(w http.ResponseWriter, r *http.Request) {
	router := mux.NewRouter()
	router.HandleFunc("/update", benchmarkTest01188Handler).Methods("POST")
	protected := csrf.Protect(benchmarkTest01188Key)(router)
	protected.ServeHTTP(w, r)
}
