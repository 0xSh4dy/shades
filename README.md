## Grammar till now
----------------------------------------------------------------------------------
### Symbols
+,-,*,/, 0..9


----------------------------------------------------------------------------------

### Backus-Naur forms
----------------------------------------------------------------------------------

1. For `2 + 3 * 5 - 8 / 3`

expression and number are non-terminal symbols.T_INTLIT is a terminal symbol.
```
expression: number 
            | expression '+' expression
            | expression '-' expression
            | expression '*' expression
            | expression '/' expression

number: T_INTLIT
```
----------------------------------------------------------------------------------
<!-- 2 * 3 + 4 * 5 -->