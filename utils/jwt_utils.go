package utils

import (
	"errors"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

func GenerateToken(email, role, secret string) (string, error) {
	claims := jwt.MapClaims{
		"email": email,
		"role":  role,
		"iss":   "booklyst-core",
		"exp":   time.Now().Add(30 * 24 * time.Hour).Unix(),
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)

	tokenString, err := token.SignedString([]byte(secret))
	if err != nil {
		return "", err
	}

	return tokenString, nil
}

func ValidateToken(r *http.Request, secret string) (string, error) {
	token, err := VerifyToken(r, secret)
	if err != nil {
		return "", err
	}

	if !token.Valid {
		return "", errors.New("invalid token")
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return "", errors.New("invalid token claims")
	}

	email, ok := claims["email"].(string)
	if !ok {
		return "", errors.New("invalid email claim")
	}

	return email, nil
}

func VerifyToken(r *http.Request, secret string) (*jwt.Token, error) {
	tokenString := ExtractTokenString(r)
	if tokenString == "" {
		return nil, errors.New("token not provided")
	}

	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return []byte(secret), nil
	})

	if err != nil {
		return nil, err
	}

	return token, nil
}

// extract token from url query or Authorization header
func ExtractTokenString(r *http.Request) string {
	token := r.URL.Query().Get("token")
	if token != "" {
		return token
	}

	tokenString := r.Header.Get("Authorization")
	if tokenString != "" {
		strArr := strings.Split(tokenString, " ")
		if len(strArr) == 2 && strings.ToLower(strArr[0]) == "bearer" {
			return strArr[1]
		}
	}

	return ""
}
