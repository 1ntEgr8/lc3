;; =============================================================
;; CS 2110 - Fall 2019
;; Project 3 - Length
;; =============================================================
;; Name: Elton Pinto
;; =============================================================

.orig x3000
     LD R0, STRING ;; loading the string pointer into R0
     AND R2, R2, #0 ;; clearing R2, which will hold the length of the string
LOOP LDR R1, R0, #0 ;; loading the character that the string pointer is pointing
                    ;; to and storing it in R1
     BRz END ;; checking if the null terminator character has been reached
     ADD R2, R2, #1 ;; incrementing R2 by 1
     ADD R0, R0, #1 ;; incrementing the string pointer
     BRnzp LOOP
END  ST R2, ANSWER ;; storing the length of the string at ANSWER
HALT

STRING  .fill x4000
ANSWER .blkw 1
.end

.orig x4000
.stringz "CS2110"
.end
