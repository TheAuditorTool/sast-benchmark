package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00763(w http.ResponseWriter, r *http.Request) {
	accountID := fmt.Sprintf("ACC-%d", rand.Int())
	_, err := DB.Exec("INSERT INTO accounts (id, email) VALUES (?, ?)",
		accountID, r.URL.Query().Get("email"))
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"account_id": accountID})
}
