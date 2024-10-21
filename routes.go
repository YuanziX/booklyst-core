package main

import (
	"net/http"

	"github.com/yuanzix/booklyst-core/controllers"
)

func InitializeRoutes(appCfg controllers.AppConfig) *http.ServeMux {
	router := http.NewServeMux()
	v1Router := http.NewServeMux()

	v1Router.HandleFunc("POST /users/signup", appCfg.PostCreateUser)
	v1Router.HandleFunc("POST /users/login", appCfg.PostLoginUser)
	v1Router.HandleFunc("DELETE /users/delete", appCfg.AuthMiddlware(appCfg.DeleteUser))

	router.Handle("/v1/", http.StripPrefix("/v1", v1Router))

	return router
}
