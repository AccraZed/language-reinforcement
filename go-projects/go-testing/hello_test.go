package main

import "testing"

func TestHello(t *testing.T) {
	verifyMessage := func(t testing.TB, got, want string) {
		t.Helper()
		if got != want {
			t.Errorf("got %q want %q", got, want)
		}
	}

	t.Run("Saying hello to a specific person", func(t *testing.T) {
		got := Hello("Amy", "")
		want := "Hello Amy!"
		verifyMessage(t, got, want)
	})

	t.Run("Saying hello with empty string param", func(t *testing.T) {
		got := Hello("", "")
		want := "Hey!"
		verifyMessage(t, got, want)
	})

	t.Run("In Spanish", func(t *testing.T) {
		got := Hello("Amy", "Spanish")
		want := "Hola Amy!"
		verifyMessage(t, got, want)
	})

	t.Run("In French", func(t *testing.T) {
		got := Hello("Amy", "French")
		want := "Bonjour Amy!"
		verifyMessage(t, got, want)
	})
}
