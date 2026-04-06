package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00574(w http.ResponseWriter, r *http.Request) {
	dn := r.FormValue("dn")
	email := r.FormValue("email")

	req := ldap.NewModifyRequest(dn, nil)
	req.Replace("mail", []string{email})

	if err := LDAPConn.Modify(req); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
