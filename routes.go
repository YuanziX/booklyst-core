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

	v1Router.HandleFunc("GET /books/{page}", appCfg.GetBooks)

	v1Router.HandleFunc("POST /cart/set", appCfg.AuthMiddlware(appCfg.PostSetItemInCart))
	v1Router.HandleFunc("GET /cart", appCfg.AuthMiddlware(appCfg.GetCartItems))

	router.Handle("/v1/", http.StripPrefix("/v1", v1Router))

	return router
}
