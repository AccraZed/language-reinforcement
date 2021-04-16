package main

const spanish = "Spanish"
const french = "French"
const helloEngPrefix = "Hello "
const helloSpaPrefix = "Hola "
const helloFrePrefix = "Bonjour "

func Hello(obj string, lang string) string {
	if obj == "" {
		return "Hey!"
	}

	return greetingPrefix(lang) + obj + "!"
}

func greetingPrefix(lang string) (prefix string) {
	switch lang {
	case spanish:
		prefix = helloSpaPrefix
	case french:
		prefix = helloFrePrefix
	default:
		prefix = helloEngPrefix
	}

	return
}
