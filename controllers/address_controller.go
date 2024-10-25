package controllers

import (
	"encoding/json"
	"net/http"

	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func (ac *AppConfig) PostCreateAddress(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	type parameters struct {
		Street  string `json:"street"`
		City    string `json:"city"`
		State   string `json:"state"`
		ZipCode string `json:"zip_code"`
		Country string `json:"country"`
	}

	decoder := json.NewDecoder(r.Body)
	params := parameters{}
	if err := decoder.Decode(&params); err != nil {
		utils.RespondError(w, http.StatusBadRequest, err.Error())
		return
	}

	address, err := ac.Query.CreateAddress(
		r.Context(),
		database.CreateAddressParams{
			UserID:  creds.ID,
			Street:  params.Street,
			City:    params.City,
			State:   params.State,
			ZipCode: params.ZipCode,
			Country: params.Country,
		},
	)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusCreated, map[string]any{
		"message":    "Address created successfully",
		"address_id": address.ID,
	})
}

func (ac *AppConfig) GetAddresses(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	addresses, err := ac.Query.GetAddressesByUserID(r.Context(), creds.ID)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, addresses)
}
