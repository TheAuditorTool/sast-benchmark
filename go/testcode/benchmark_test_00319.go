package testcode

import (
	beego "github.com/beego/beego/v2/server/web"
)

type BenchmarkTest00319Controller struct {
	beego.Controller
}

func (c *BenchmarkTest00319Controller) Get() {
	name := c.Ctx.Input.Param(":name")
	query := "SELECT * FROM users WHERE name = '" + name + "'"
	var result string
	err := DB.QueryRow(query).Scan(&result)
	if err != nil {
		c.Data["json"] = map[string]string{"error": err.Error()}
		c.ServeJSON()
		return
	}
	c.Data["json"] = map[string]string{"result": result}
	c.ServeJSON()
}
