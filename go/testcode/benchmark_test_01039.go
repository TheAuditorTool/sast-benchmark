package testcode

import (
	"fmt"
	"net/http"
	"strconv"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01039Conn *ldap.Conn

func BenchmarkTest01039(w http.ResponseWriter, r *http.Request) {
	empIDStr := r.URL.Query().Get("emp_id")
	empID, err := strconv.Atoi(empIDStr)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	filter := fmt.Sprintf("(employeeNumber=%d)", empID)
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01039Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}
