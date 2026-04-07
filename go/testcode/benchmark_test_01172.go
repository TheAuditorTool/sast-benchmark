package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01172(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	resourceID := strings.TrimPrefix(r.URL.Path, "/tenant/resources/")

	var tenantID, userID string
	err := DB.QueryRow(
		"SELECT u.tenant_id, u.id FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?",
		sessionToken,
	).Scan(&tenantID, &userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var name, data string
	err = DB.QueryRow(
		"SELECT name, data FROM resources WHERE id = ? AND tenant_id = ?",
		resourceID, tenantID,
	).Scan(&name, &data)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"name": name, "data": data})
}
