package testcode

import (
	"net/http"
	"os/exec"

	"github.com/labstack/echo/v4"
)

func BenchmarkTest00300(ctx echo.Context) error {
	cmd := ctx.QueryParam("cmd")
	output, err := exec.Command("sh", "-c", cmd).CombinedOutput()
	if err != nil {
		return ctx.JSON(http.StatusInternalServerError, map[string]string{"error": err.Error()})
	}
	return ctx.JSON(http.StatusOK, map[string]string{"output": string(output)})
}
