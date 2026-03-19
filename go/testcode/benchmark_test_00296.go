package testcode

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func BenchmarkTest00296(c *gin.Context) {
	id := c.Query("id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	defer rows.Close()
	c.JSON(http.StatusOK, gin.H{"status": "ok"})
}
