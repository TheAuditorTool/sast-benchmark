package testcode

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func BenchmarkTest00294(c *gin.Context) {
	name := c.Param("name")
	query := "SELECT * FROM users WHERE name = '" + name + "'"
	row := DB.QueryRow(query)
	var id int
	var username, email string
	_ = row.Scan(&id, &username, &email)
	c.JSON(http.StatusOK, gin.H{"id": id, "name": username})
}
