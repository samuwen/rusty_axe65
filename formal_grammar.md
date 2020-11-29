<program> ::= { <statement> }
<statement> ::= <assignment> | <directive> | <label> | <opcode>

--------------- ASSIGNMENTS -----------------
<assignment> ::= <id> "=" <expression>

--------------- DIRECTIVES -------------------
<directive> ::= <dir-segment> | <dir-other>
<dir-segment> ::= ".segment" <dir-seg-name>
<dir-seg-name> ::= <double-quote> <up-case-letter> { <up-case-letter> } <double-quote>
<dir-other> ::= <dir-name> { <dir-arg> }
<dir-name> ::= "." <low-case-letter> { <low-case-letter> }
<dir-arg> ::= (<string-const>|<expression>) { "," <dir-arg> }
<string-const> ::= <dir-string-arg> | <dir-value>
<dir-string-arg> ::= <double-quote> <letter> { (<letter>|<symbol>) } <double-quote>
<dir-value> ::= <single-quote> <letter> <single-quote> | <number>

--------------- LABELS -----------------------
<label> ::= <normal-label> | <local-label> | <unnamed-label>
<normal-label> ::= <id> ":"
<local-label> ::= "@" <id> ":"
<unnamed-label> ::= ":"

--------------- OPCODE -----------------------
<opcode> ::= <accumulator-mode> | <immediate-mode> | <direct-memory-mode> | <indirect-memory-mode>
<accumulator-mode> ::= <op-id>
<immediate-mode> ::= <op-id> "#" <expression>
<direct-memory-mode> ::= <op-id> <expression>
<indirect-memory-mode> ::= <indirect-x> | <indirect-y>
<indirect-x> ::= <op-id> "(" <number> "," "X" ")"
<indirect-y> ::= <op-id> "(" <number> ")" "," "Y"

--------------- EXPRESSIONS -----------------
<expression> ::= "!"|"NOT" <expression> | <bool-not-exp>
<bool-not-exp> ::= <bool-or-exp> { ("||"|"OR") <bool-or-exp> }
<bool-or-exp> ::= <bool-xor-and-exp> { ("&&"|"XOR"|"AND") <bool-xor-and-exp> }
<bool-xor-and-exp> ::= <relational-exp> { ("="|"<>"|"<"|">"|"<="|">=") <relational-exp> }
<relational-exp> ::= <binary-add-sub-exp> { ("+"|"-"|"|"|"BITOR") <binary-add-sub-exp> }
<binary-add-sub-exp> ::= <bitwise-mul-div-exp> { ("\*"|"/"|"<<"|">>"|"^"|"&"|"MOD"|"BITAND"|"BITXOR"|"SHL"|"SHR") <bitwise-mul-div-exp> }
<bitwise-mul-div-exp> ::= <unary-op> <bitwise-mul-div-exp> | <unary-exp>
<unary-exp> ::= <factor> { (".CONCAT") <factor> }
<factor> ::= "(" <expression> ")" | <id> | <number> | <ulabel>
<unary-op> ::= ("^"|">"|"<"|"~"|"+"|"-"|<built-in-pseudo-variable>|<built-in-pseudo-function>|"BITNOT")
<built-in-pseudo-variable> ::= ("\_"|"ASIZE"|"CPU"|"ISIZE"|"PARAMCOUNT"|"TIME"|"VERSION")
<built-in-pseudo-function> ::= ("ADDRSIZE"|"BANK"|"BANKBYTE"|"BLANK"|"CONST"|"HIBYTE"|"HIWORD"|"IDENT"|"LEFT"|"LOBYTE"|"LOWORD"|"MATCH"|"MAX"|"MID"|"MIN"|"REF"|"REFERENCED"|"RIGHT"|"SIZEOF"|"STRAT"|"SPRINTF"|"STRING"|"STRLEN"|"TCOUNT"|"XMATCH")

--------------- GENERIC ---------------------
<id> ::= <letter> { <id-string> }
<id-string> ::= <letter> | "_" | <digit>
<number> ::= ("#"|"%"|"") <digit> { <digit> }
<letter> ::= <low-case-letter> | <up-case-letter>
<low-case-letter> ::= "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
<up-case-letter> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<symbol> ::= "|" | "!" | "#" | "$" | "%" | "&" | "(" | ")" | "\*" | "+" | "," | "-" | "." | "/" | ":" | ";" | ">" | "=" | "<" | "?" | "@" | "[" | "\" | "]" | "^" | "_" | "`" | "{" | "}" | "~"
<single-quote> ::= "\'"
<double-quote> ::= "\""
<ulabel> ::= ":" ("-"|"+") { ("-"|"+") }
