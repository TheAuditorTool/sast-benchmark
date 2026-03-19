package testcode

import (
	"net/http"
	"os/exec"

	"github.com/gin-gonic/gin"
)

func BenchmarkTest00297(c *gin.Context) {
	host := c.Query("host")
	output, err := exec.Command("ping", "-c", "1", host).Output()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}
