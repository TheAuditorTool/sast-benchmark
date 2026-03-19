package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00361(w http.ResponseWriter, r *http.Request) {
	searchTerm := r.URL.Query().Get("q")
	filter := "(cn=*" + searchTerm + "*)"
	searchReq := ldap.NewSearchRequest("dc=example,dc=com", ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false, filter, []string{"dn", "cn", "mail"}, nil)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	names := make([]string, 0, len(result.Entries))
	for _, entry := range result.Entries {
		names = append(names, entry.GetAttributeValue("cn"))
	}
	RespondJSON(w, http.StatusOK, map[string]interface{}{"results": names})
}
