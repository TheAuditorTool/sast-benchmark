package testcode

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func BenchmarkTest00293(c *gin.Context) {
	id := c.Query("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()
	c.JSON(http.StatusOK, gin.H{"status": "ok"})
}
