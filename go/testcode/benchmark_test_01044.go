package testcode

import (
	"net/http"
	"os"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest01044(w http.ResponseWriter, r *http.Request) {
	username := r.FormValue("username")
	password := r.FormValue("password")
	conn, err := ldap.DialURL(os.Getenv("LDAP_URL"))
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	defer conn.Close()
	userDN := "uid=" + ldap.EscapeFilter(username) + ",ou=users,dc=example,dc=com"
	if err := conn.Bind(userDN, password); err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "authenticated"})
}
