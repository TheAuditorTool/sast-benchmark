package testcode

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
)

func BenchmarkTest00553(w http.ResponseWriter, r *http.Request) {
	vaultAddr := os.Getenv("VAULT_ADDR")
	vaultToken := os.Getenv("VAULT_TOKEN")

	req, err := http.NewRequest("GET", vaultAddr+"/v1/secret/data/database", nil)
	if err != nil {
		http.Error(w, "vault request error", http.StatusInternalServerError)
		return
	}
	req.Header.Set("X-Vault-Token", vaultToken)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		http.Error(w, "vault unreachable", http.StatusServiceUnavailable)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var vaultResp struct {
		Data struct {
			Data struct {
				Username string `json:"username"`
				Password string `json:"password"`
			} `json:"data"`
		} `json:"data"`
	}
	if err := json.Unmarshal(body, &vaultResp); err != nil {
		http.Error(w, "vault parse error", http.StatusInternalServerError)
		return
	}

	dsn := fmt.Sprintf("%s:%s@tcp(db.internal:3306)/appdb",
		vaultResp.Data.Data.Username,
		vaultResp.Data.Data.Password,
	)
	db, err := sql.Open("mysql", dsn)
	if err != nil {
		http.Error(w, "db open error", http.StatusInternalServerError)
		return
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		http.Error(w, "db ping failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "connected"})
}
