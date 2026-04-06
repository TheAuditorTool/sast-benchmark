package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00576(w http.ResponseWriter, r *http.Request) {
	dept := r.URL.Query().Get("department")
	filter := fmt.Sprintf("(&(objectClass=person)(department=%s))", dept)
	searchReq := ldap.NewSearchRequest(
		"ou=people,dc=example,dc=com",
		ldap.ScopeWholeSubtree,
		ldap.NeverDerefAliases,
		0, 0, false,
		filter,
		[]string{"dn", "cn", "uid", "department"},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
