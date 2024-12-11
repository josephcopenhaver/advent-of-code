package main

import (
	_ "embed"
	"regexp"
	"strconv"
)

//go:embed input.txt
var INPUT []byte

var re = regexp.MustCompile(`mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)`)

func main() {
	var sum int

	for _, v := range re.FindAllSubmatch(INPUT, -1) {
		a, err := strconv.Atoi(string(v[1]))
		if err != nil {
			panic(err)
		}

		b, err := strconv.Atoi(string(v[2]))
		if err != nil {
			panic(err)
		}

		sum += a * b
	}

	println(sum)
}
