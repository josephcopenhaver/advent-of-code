package main

import (
	"bufio"
	"bytes"
	_ "embed"
	"strings"
	"unicode/utf8"
)

//go:embed input.txt
var INPUT []byte

const (
	NEEDLE           = "MAS"
	NEEDLE_IDX_SHIFT = len(NEEDLE) - 1
)

func reverseStr(s string) string {
	numBytes := len(s)
	bytesBuf := make([]byte, numBytes)
	for i := 0; i < numBytes; {
		r, n := utf8.DecodeRuneInString(s[i:])
		i += n
		utf8.EncodeRune(bytesBuf[numBytes-i:], r)
	}
	return string(bytesBuf)
}

func main() {
	var sum int
	buf := make([]byte, 0, len(NEEDLE))
	needle_rev := reverseStr(NEEDLE)

	h := strings.Count(string(INPUT), "\n") + 1
	sc := bufio.NewScanner(bytes.NewReader(INPUT))
	var grid [][]byte
	for sc.Scan() {
		s := sc.Text()
		grid = append(grid, []byte(s))
	}
	w := len(grid[0])

	// diagonals
	for y := range h - NEEDLE_IDX_SHIFT {
		negative_trend_y := y + NEEDLE_IDX_SHIFT
		for x := range w - NEEDLE_IDX_SHIFT {
			// positive y trend
			buf = buf[:0]
			for d := range len(NEEDLE) {
				buf = append(buf, grid[y+d][x+d])
			}
			s := string(buf)
			if !(s == NEEDLE || s == needle_rev) {
				continue
			}

			// negative y trend
			buf = buf[:0]
			for d := range len(NEEDLE) {
				buf = append(buf, grid[negative_trend_y-d][x+d])
			}
			s = string(buf)
			if s == NEEDLE || s == needle_rev {
				sum++
			}
		}
	}

	println(sum)
}
