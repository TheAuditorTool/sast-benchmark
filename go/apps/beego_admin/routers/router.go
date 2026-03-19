package routers

import (
	beego "github.com/beego/beego/v2/server/web"
	"github.com/theauditor/beego_admin/controllers"
)

// Init initializes all routes
func Init() {
	// Namespace for API routes
	ns := beego.NewNamespace("/api",
		// User routes - vulnerable
		beego.NSRouter("/users/:id", &controllers.UserController{}, "get:GetUser"),
		beego.NSRouter("/users/by-name", &controllers.UserController{}, "get:GetUserByUsername"),
		beego.NSRouter("/users/search", &controllers.UserController{}, "get:SearchUsers"),
		beego.NSRouter("/users", &controllers.UserController{}, "post:CreateUser"),
		beego.NSRouter("/users/:id", &controllers.UserController{}, "put:UpdateUser"),
		beego.NSRouter("/users/:id", &controllers.UserController{}, "delete:DeleteUser"),

		// User routes v2 - multi-hop via service
		beego.NSRouter("/users/v2", &controllers.UserController{}, "post:CreateUserViaService"),
		beego.NSRouter("/users/v2/search", &controllers.UserController{}, "get:SearchViaService"),

		// Reports - multi-hop with multiple sinks
		beego.NSRouter("/reports", &controllers.UserController{}, "post:RunReportViaService"),

		// Auth routes - header/cookie sources
		beego.NSRouter("/auth/header", &controllers.UserController{}, "get:AuthByHeader"),
		beego.NSRouter("/auth/cookie", &controllers.UserController{}, "get:AuthByCookie"),

		// Admin routes - command injection
		beego.NSRouter("/admin/cmd", &controllers.UserController{}, "post:RunSystemCommand"),
		beego.NSRouter("/admin/shell", &controllers.UserController{}, "post:RunShellCommand"),

		// File routes - path traversal
		beego.NSRouter("/files/download", &controllers.UserController{}, "get:DownloadFile"),
		beego.NSRouter("/files/upload", &controllers.UserController{}, "post:UploadFile"),
		beego.NSRouter("/config/:name", &controllers.UserController{}, "get:ReadConfig"),

		// Secure routes - for comparison
		beego.NSRouter("/users/secure/:id", &controllers.UserController{}, "get:GetUserSecure"),
	)

	beego.AddNamespace(ns)
}
