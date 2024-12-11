package main

import (
	_ "embed"
	"regexp"
	"strconv"
	"strings"
)

//go:embed input.txt
var INPUT []byte

var re = regexp.MustCompile(`mul\((-?[1-9][0-9]*),(-?[1-9][0-9]*)\)`)

const (
	ON  = "do()"
	OFF = "don't()"
)

func main() {
	var sum int
	var buf []byte

	if idx := strings.Index(string(INPUT), OFF); idx == -1 {
		buf = INPUT
	} else {
		buf = make([]byte, 0, len(INPUT))
		buf = append(buf, INPUT[:idx]...)

		onParts := strings.Split(string(INPUT[idx+len(OFF):]), ON)
		onParts = onParts[1:]

		for _, v := range onParts {
			v, _, _ := strings.Cut(v, OFF)

			buf = append(buf, ' ')
			buf = append(buf, []byte(v)...)
		}
	}

	for _, v := range re.FindAllSubmatch(buf, -1) {
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
