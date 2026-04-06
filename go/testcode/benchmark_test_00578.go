package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00578(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	safeUser := ldap.EscapeFilter(username)
	filter := "(uid=" + safeUser + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeSingleLevel,
		ldap.NeverDerefAliases,
		1, 0, false,
		filter,
		[]string{"dn"},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil || len(result.Entries) == 0 {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	serverDN := result.Entries[0].DN
	modReq := ldap.NewModifyRequest(serverDN, nil)
	modReq.Replace("pwdReset", []string{"TRUE"})

	if err := LDAPConn.Modify(modReq); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
