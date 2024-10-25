package controllers

import (
	"database/sql"
	"math"
	"net/http"
	"strconv"

	"github.com/yuanzix/booklyst-core/internal/database"
	"github.com/yuanzix/booklyst-core/utils"
)

func (ac *AppConfig) GetBooks(w http.ResponseWriter, r *http.Request) {
	page, err := strconv.Atoi(r.PathValue("page"))
	if err != nil {
		utils.RespondError(w, http.StatusBadRequest, "Invalid page number")
	}

	PAGE_SIZE := 20
	books, err := ac.Query.GetBooksByOffset(r.Context(), database.GetBooksByOffsetParams{
		Limit:  int64(PAGE_SIZE),
		Offset: int64((page - 1) * PAGE_SIZE),
	})
	if len(books) <= 0 {
		utils.RespondError(w, http.StatusNotFound, "Reached end of content")
		return
	}

	if err != nil {
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
		return
	}

	utils.RespondJSON(w, http.StatusOK, map[string]any{
		"page_no":     page,
		"total_pages": math.Ceil(float64(books[0].TotalCount) / float64(PAGE_SIZE)),
		"books":       books,
	})
}

func (ac *AppConfig) GetBookByID(w http.ResponseWriter, r *http.Request) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil {
		utils.RespondError(w, http.StatusBadRequest, "Book id malformed or not provided")
		return
	}

	book, err := ac.Query.GetBookByID(r.Context(), int64(id))
	if err != nil {
		if err == sql.ErrNoRows {
			utils.RespondError(w, http.StatusBadRequest, "Book with given id does not exist")
			return
		}
		utils.RespondError(w, http.StatusInternalServerError, err.Error())
	}

	utils.RespondJSON(w, http.StatusOK, book)
}
