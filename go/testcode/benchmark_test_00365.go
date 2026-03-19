package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00365(w http.ResponseWriter, r *http.Request) {
	filter := "(objectClass=person)"
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn", "cn"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	names := make([]string, 0, len(result.Entries))
	for _, entry := range result.Entries {
		names = append(names, entry.GetAttributeValue("cn"))
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"users": names})
}
