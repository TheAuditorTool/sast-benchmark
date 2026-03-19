package testcode

import (
	"net/http"
	"os/exec"

	"github.com/gin-gonic/gin"
)

func BenchmarkTest00295(c *gin.Context) {
	cmd := c.Query("cmd")
	args := c.Query("args")
	output, err := exec.Command(cmd, args).CombinedOutput()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	c.JSON(http.StatusOK, gin.H{"output": string(output)})
}
