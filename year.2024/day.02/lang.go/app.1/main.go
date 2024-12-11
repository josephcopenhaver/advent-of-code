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

func mono_inc(v []int) bool {
	for i := 1; i < len(v); i++ {
		if v[i-1] >= v[i] {
			return false
		}
		if v[i]-v[i-1] > 3 {
			return false
		}
	}

	return true
}

func mono_dec(v []int) bool {
	for i := 1; i < len(v); i++ {
		if v[i-1] <= v[i] {
			return false
		}
		if v[i-1]-v[i] > 3 {
			return false
		}
	}

	return true
}

func main() {
	var sum int

	var buf []int
	sc := bufio.NewScanner(bytes.NewReader(INPUT))
	for sc.Scan() {
		buf = buf[:0]

		for _, v := range strings.Split(sc.Text(), " ") {
			n, err := strconv.Atoi(v)
			if err != nil {
				panic(err)
			}
			buf = append(buf, n)
		}

		if mono_inc(buf) || mono_dec(buf) {
			sum++
		}
	}

	println(sum)
}
