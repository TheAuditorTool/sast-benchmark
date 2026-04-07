package testcode

import (
	"net/http"

	"github.com/gorilla/csrf"
	"github.com/gorilla/mux"
)

var benchmarkTest01201Key = []byte("gorilla-csrf-protect-32-byte-key")

func benchmarkTest01201FormHandler(w http.ResponseWriter, r *http.Request) {
	token := csrf.Token(r)
	RespondJSON(w, http.StatusOK, map[string]string{"csrf_token": token})
}

func benchmarkTest01201SubmitHandler(w http.ResponseWriter, r *http.Request) {
	userID := r.FormValue("user_id")
	content := r.FormValue("content")
	_, err := DB.Exec("INSERT INTO posts (user_id, content) VALUES (?, ?)", userID, content)
	if err != nil {
		http.Error(w, "insert failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "post created"})
}

func BenchmarkTest01201(w http.ResponseWriter, r *http.Request) {
	router := mux.NewRouter()
	router.HandleFunc("/form", benchmarkTest01201FormHandler).Methods("GET")
	router.HandleFunc("/submit", benchmarkTest01201SubmitHandler).Methods("POST")
	csrf.Protect(benchmarkTest01201Key)(router).ServeHTTP(w, r)
}
