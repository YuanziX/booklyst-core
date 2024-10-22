package controllers

import (
	"database/sql"
	"encoding/json"
	"net/http"

	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func (ac *AppConfig) GetCartItems(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	cartItems, err := ac.DB.GetCartItemsByUserID(r.Context(), creds.ID)
	if err != nil {
		if err == sql.ErrNoRows {
			utils.RespondJSON(
				w, http.StatusOK, map[string]any{
					"item_count": 0,
					"cart_items": []database.CartItem{},
				},
			)
			return
		}
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(
		w, http.StatusOK, map[string]any{
			"item_count": len(cartItems),
			"cart_items": cartItems,
		},
	)
}

func (ac *AppConfig) PostSetItemInCart(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	type parameters struct {
		BookId   int64 `json:"book_id"`
		Quantity int64 `json:"quantity"`
	}
	decoder := json.NewDecoder(r.Body)
	params := parameters{}
	if err := decoder.Decode(&params); err != nil {
		utils.RespondError(w, http.StatusBadRequest, "Invalid request payload")
		return
	}

	if params.Quantity < 0 {
		utils.RespondError(w, http.StatusBadRequest, "Quantity can not be negative")
		return
	}

	if params.Quantity == 0 {
		err := ac.DB.RemoveItemFromCart(r.Context(), database.RemoveItemFromCartParams{
			UserID: creds.ID,
			BookID: params.BookId,
		})
		if err != nil {
			utils.RespondError(w, http.StatusInternalServerError, err.Error())
			return
		}
		utils.RespondJSON(w, http.StatusOK, map[string]string{
			"message": "successfully deleted cart item",
		})
		return
	}

	err := ac.DB.AddItemToCart(r.Context(), database.AddItemToCartParams{
		UserID: creds.ID,
		BookID: params.BookId,
		Quantity: sql.NullInt64{
			Int64: params.Quantity,
			Valid: true,
		},
	})
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, map[string]string{
		"message": "successfully modified cart item",
	})
}
