package controllers

import (
	"database/sql"
	"encoding/json"
	"net/http"

	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func (ac *AppConfig) PostCreateOrder(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	type Parameters struct {
		AddressId int64 `json:"address_id"`
	}

	decoder := json.NewDecoder(r.Body)
	params := Parameters{}

	if err := decoder.Decode(&params); err != nil {
		utils.RespondError(w, http.StatusBadRequest, err.Error())
		return
	}

	tx, err := ac.DB.Begin()
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	defer tx.Rollback()

	qtx := ac.Query.WithTx(tx)

	address, err := qtx.GetAddressByID(r.Context(), params.AddressId)
	if err != nil {
		if err == sql.ErrNoRows {
			utils.RespondError(w, http.StatusNotFound, "address with provided id not found")
			return
		}
		utils.RespondError(w, http.StatusNotFound, err.Error())
		return
	}

	books, err := qtx.GetCartItemsByUserID(r.Context(), creds.ID)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	if len(books) == 0 {
		utils.RespondError(w, http.StatusBadRequest, "the user's cart is empty")
		return
	}

	total := 0.0
	for _, book := range books {
		total += book.Price * float64(book.Quantity.Int64)
	}

	order, err := qtx.CreateOrder(r.Context(), database.CreateOrderParams{
		UserID:     creds.ID,
		AddressID:  address.ID,
		TotalPrice: total,
	})
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	for _, book := range books {
		err := qtx.CreateOrderItems(r.Context(), database.CreateOrderItemsParams{
			OrderID:      order.ID,
			BookID:       book.BookID,
			Quantity:     book.Quantity,
			PricePerItem: book.Price,
		})
		if err != nil {
			utils.RespondError(w, http.StatusInternalServerError, err.Error())
			return
		}
	}

	err = qtx.ClearCart(r.Context(), creds.ID)
	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	if err = tx.Commit(); err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusCreated, map[string]any{
		"message":  "order created successfully",
		"order_id": order.ID,
	})
}

func (ac *AppConfig) GetOrders(w http.ResponseWriter, r *http.Request) {
	creds, ok := r.Context().Value(CredsContextKey).(Creds)
	if !ok {
		utils.RespondError(w, http.StatusInternalServerError, "Could not retrieve credentials")
		return
	}

	orders, err := ac.Query.GetOrderByUserId(r.Context(), creds.ID)
	if err != nil {
		if err == sql.ErrNoRows {
			utils.RespondError(w, http.StatusNotFound, "no orders found")
		}
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, orders)
}
