package testcode

import (
	"os/exec"

	beego "github.com/beego/beego/v2/server/web"
)

type BenchmarkTest00321Controller struct {
	beego.Controller
}

func (c *BenchmarkTest00321Controller) Get() {
	host := c.GetString("host")
	out, err := exec.Command("ping", "-c", "1", host).Output()
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	c.Data["json"] = map[string]string{"output": string(out)}
	c.ServeJSON()
}
