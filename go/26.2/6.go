package main

import (
	"fmt"
)

func reclassify(planets *[]string) {
	*planets = (*planets)[0:8]
}

func main() {
	planets := []string{
		"Mercury", "Venus", "Earth", "Mars",
		"Jupiter", "Saturn", "Uranus", "Neptune",
		"Pluto", "Eris", "Haumea", "Makemake",
	}

	reclassify(&planets)
	fmt.Println(planets)
}
