package main

import (
	_ "embed"
)

//go:embed input.txt
var INPUT []byte

func main() {
	var sum int

	// sc := bufio.NewScanner(bytes.NewReader(INPUT))

	println(sum)
}
