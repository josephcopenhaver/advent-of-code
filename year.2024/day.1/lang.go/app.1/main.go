package main

import (
	"bufio"
	"bytes"
	_ "embed"
	"slices"
	"strconv"
	"strings"
)

//go:embed input.txt
var INPUT []byte

func main() {
	size := strings.Count(string(INPUT), "\n") + 1

	left := make([]int, 0, size)
	right := make([]int, 0, size)
	{
		sc := bufio.NewScanner(bytes.NewReader(INPUT))

		i := -1
		for sc.Scan() {
			i++

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
			right = append(right, v)
		}
	}

	slices.Sort(left)
	slices.Sort(right)

	var sum int
	for i := 0; i < size; i++ {
		d := right[i] - left[i]
		if d < 0 {
			d = -d
		}
		sum += d
	}
	println(sum)
}
