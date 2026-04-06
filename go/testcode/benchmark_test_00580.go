package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest00580AllowedAttrs = map[string]bool{
	"cn":         true,
	"mail":       true,
	"uid":        true,
	"department": true,
}

func BenchmarkTest00580(w http.ResponseWriter, r *http.Request) {
	department := r.URL.Query().Get("department")
	attr := r.URL.Query().Get("attr")

	if !benchmarkTest00580AllowedAttrs[attr] {
		http.Error(w, "invalid attribute", http.StatusBadRequest)
		return
	}

	safeDept := ldap.EscapeFilter(department)
	filter := fmt.Sprintf("(&(objectClass=person)(department=%s))", safeDept)
	searchReq := ldap.NewSearchRequest(
		"ou=people,dc=example,dc=com",
		ldap.ScopeWholeSubtree,
		ldap.NeverDerefAliases,
		0, 0, false,
		filter,
		[]string{attr},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
