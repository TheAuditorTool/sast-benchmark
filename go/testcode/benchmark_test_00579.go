package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest00579ScopeMap = map[string]int{
	"base": ldap.ScopeBaseObject,
	"one":  ldap.ScopeSingleLevel,
	"sub":  ldap.ScopeWholeSubtree,
}

func BenchmarkTest00579(w http.ResponseWriter, r *http.Request) {
	name := r.URL.Query().Get("name")
	scopeName := r.URL.Query().Get("scope_name")

	scope, ok := benchmarkTest00579ScopeMap[scopeName]
	if !ok {
		scope = ldap.ScopeWholeSubtree
	}

	safe := ldap.EscapeFilter(name)
	filter := "(cn=" + safe + ")"
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
