package controllers

import (
	"database/sql"
	"encoding/json"
	"net/http"

	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func (ac *AppConfig) PostCreateUser(w http.ResponseWriter, r *http.Request) {
	type parameters struct {
		FirstName string `json:"first_name"`
		LastName  string `json:"last_name"`
		Email     string `json:"email"`
		Password  string `json:"password"`
		Role      string `json:"role"`
	}

	decoder := json.NewDecoder(r.Body)
	params := parameters{}
	if err := decoder.Decode(&params); err != nil {
		utils.RespondError(w, http.StatusBadRequest, "Invalid request payload")
		return
	}

	_, err := ac.DB.GetUserByEmail(r.Context(), params.Email)
	if err != sql.ErrNoRows {
		if err == nil {
			utils.RespondError(w, http.StatusConflict, "User already exists")
			return
		}
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	passwordHash, err := utils.HashPassword(params.Password)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	user, err := ac.DB.CreateUser(r.Context(), database.CreateUserParams{
		FirstName:    params.FirstName,
		LastName:     params.LastName,
		Email:        params.Email,
		PasswordHash: passwordHash,
		Role:         params.Role,
	})
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusCreated, user)
}

func (ac *AppConfig) PostLoginUser(w http.ResponseWriter, r *http.Request) {
	type parameters struct {
		Email    string `json:"email"`
		Password string `json:"password"`
	}

	decoder := json.NewDecoder(r.Body)
	params := parameters{}
	if err := decoder.Decode(&params); err != nil {
		utils.RespondError(w, http.StatusBadRequest, "Invalid request payload")
		return
	}

	user, err := ac.DB.GetUserIdRoleAndPasswordByEmail(r.Context(), params.Email)
	if err != nil {
		if err == sql.ErrNoRows {
			utils.RespondError(w, http.StatusNotFound, "User not found")
			return
		}

		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	if err = utils.CompareHashAndPassword(user.PasswordHash, params.Password); err != nil {
		utils.RespondError(w, http.StatusUnauthorized, "Invalid password")
		return
	}

	token, err := utils.GenerateToken(params.Email, user.Role, ac.JWT_SECRET)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, map[string]string{"token": token})
}

func (ac *AppConfig) DeleteUser(w http.ResponseWriter, r *http.Request) {
	creds := r.Context().Value(CredsContextKey).(Creds)
	err := ac.DB.DeleteUserByID(r.Context(), creds.ID)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, map[string]string{"message": "User deleted"})
}
