#lang racket
(define (puzzle-read filename)
  (with-input-from-file filename
    (lambda () (for/list ([line (in-lines)]) line))))
(let ([input (puzzle-read "day1.test.input")]))