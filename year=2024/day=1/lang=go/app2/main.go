package main

import (
	"bufio"
	"bytes"
	_ "embed"
	"strconv"
	"strings"
)

//go:embed input.txt
var INPUT []byte

func main() {
	size := 1
	{
		for _, b := range INPUT {
			if b == '\n' {
				size++
			}
		}
	}

	left := make([]int, 0, size)
	m := make(map[int]int, size) // note: oversized but that's fine
	{
		sc := bufio.NewScanner(bytes.NewReader(INPUT))

		for sc.Scan() {

			fields := strings.Fields(sc.Text())

			v, err := strconv.Atoi(fields[0])
			if err != nil {
				panic(err)
			}
			left = append(left, v)

			v, err = strconv.Atoi(fields[1])
			if err != nil {
				panic(err)
			}
			m[v] += v
		}
	}

	var sum int
	for _, v := range left {
		sum += m[v]
	}
	println(sum)
}
