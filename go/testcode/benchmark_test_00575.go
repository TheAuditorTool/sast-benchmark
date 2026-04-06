package testcode

import (
	"net/http"
	"strconv"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00575(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	scopeStr := r.URL.Query().Get("scope")

	scope, err := strconv.Atoi(scopeStr)
	if err != nil {
		scope = ldap.ScopeWholeSubtree
	}

	filter := "(cn=" + name + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		scope,
		ldap.NeverDerefAliases,
		0, 0, false,
		filter,
		[]string{"dn", "cn", "mail"},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
