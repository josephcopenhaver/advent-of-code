package main

import (
	_ "embed"
	"strings"
)

//go:embed input.txt
var INPUT []byte

func main() {
	var sum int
	size := strings.Count(string(INPUT), "\n") + 1

	// sc := bufio.NewScanner(bytes.NewReader(INPUT))

	println(sum)
}
