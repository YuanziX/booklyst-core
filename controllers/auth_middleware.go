package controllers

import (
	"context"
	"net/http"

	"github.com/yuanzix/booklyst-core/utils"
)

// objects to attach the credentials to the request context
type contextKey string

const CredsContextKey contextKey = "creds"

type Creds struct {
	ID    int64
	Email string
	Role  string
}

func (ac *AppConfig) AuthMiddlware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		email, err := utils.ValidateToken(r, ac.JWT_SECRET)
		if err != nil {
			utils.RespondError(w, http.StatusUnauthorized, "Unauthorized")
			return
		}

		res, err := ac.Query.GetUserIdRoleAndPasswordByEmail(r.Context(), email)
		if err != nil {
			utils.RespondError(w, http.StatusInternalServerError, err.Error())
			return
		}

		creds := Creds{
			ID:    res.ID,
			Email: email,
			Role:  res.Role,
		}

		next(w, r.WithContext(context.WithValue(r.Context(), CredsContextKey, creds)))
	}
}
