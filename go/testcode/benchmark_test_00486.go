package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00486(w http.ResponseWriter, r *http.Request) {
	attrs := []string{"dn", "cn", "mail"}
	requestedAttr := r.URL.Query().Get("attr")
	if requestedAttr != "" {
		attrs = append(attrs, requestedAttr)
	}
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree,
		ldap.NeverDerefAliases,
		100, 0, false,
		"(objectClass=person)",
		attrs,
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
