package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01035Conn *ldap.Conn

func BenchmarkTest01035(w http.ResponseWriter, r *http.Request) {
	userDN := r.FormValue("dn")
	oldPw := r.FormValue("old_password")
	newPw := r.FormValue("new_password")
	pwReq := ldap.NewPasswordModifyRequest(userDN, oldPw, newPw)
	if _, err := benchmarkTest01035Conn.PasswordModify(pwReq); err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "changed"})
}
