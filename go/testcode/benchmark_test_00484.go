package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

func BenchmarkTest00484(w http.ResponseWriter, r *http.Request) {
	department := r.URL.Query().Get("department")
	baseDN := "ou=" + department + ",dc=example,dc=com"
	searchReq := ldap.NewSearchRequest(
		baseDN,
		ldap.ScopeWholeSubtree,
		ldap.NeverDerefAliases,
		0, 0, false,
		"(objectClass=person)",
		[]string{"dn", "cn", "mail", "title"},
		nil,
	)
	result, err := LDAPConn.Search(searchReq)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	var entries []map[string]string
	for _, entry := range result.Entries {
		entries = append(entries, map[string]string{
			"dn":   entry.DN,
			"cn":   entry.GetAttributeValue("cn"),
			"mail": entry.GetAttributeValue("mail"),
		})
	}
	RespondJSON(w, http.StatusOK, entries)
}
