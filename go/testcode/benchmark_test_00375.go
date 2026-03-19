package testcode

import (
	"fmt"
	"net/http"

	"gorm.io/gorm"
)

var GormDB *gorm.DB

func BenchmarkTest00375(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var result map[string]interface{}
	GormDB.Raw(fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)).Scan(&result)
	RespondJSON(w, http.StatusOK, result)
}
