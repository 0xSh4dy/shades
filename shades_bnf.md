## Grammar till now
----------------------------------------------------------------------------------
### Symbols
+,-,*,/, 0..9


----------------------------------------------------------------------------------

### Backus-Naur forms (BNF)
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

2. Adding Operator Precedence
```
expression: additive_expression;
additive_expression:
    multiplicative_expression
    | additive_expression + multiplicative_expression
    | additive_expression - multiplicative_expression
    ;

multiplicative_expression:
    number
    | number '*' multiplicative_expression
    | number '/' multiplicative_expression
    ;
number: T_INTLIT;
```
Consider a node let's say Node1 of the type arithmetic operator. An operator having higher precedence than the operator stored in Node1 must be stored in a Node, let's say Node2 which would either be a part of the left subtree or the right subtree. Operators with lower precedence must be stored at higher positions in the tree. additive_expression would call multiplicative_expression recursively.
This recursive descent method is really really bad due to the extremely large number of recursive calls. This is where `Pratt Parsing` comes into play. If recursive descent is peanut butter, Pratt parsing is the jelly. When you mix the two together, you get a simple, terse, readable parser that can handle any grammar you throw at it.

----------------------------------------------------------------------------------

### Keywords and identifiers
Keywords and identifiers must either start with an alphabet or with an underscore


### BNF for expressions
```
print 1+2+3+4;
print 1*2*3+4;
```
There can be multiple expressions in the same file. So, we get
```
compound_statement: '{' '}' // empty
    | '{' statement '}'
    | '{' statement statements '}'
    ;
statements: statement
    | statement statements
    ;
statement: 'print' expression ';'
    | `var` identifier ';'
    | identifier '=' expression ';'
    | if_statement
    ;

if_statement: if_head
    | if_head 'else' compound_statement
    ;
if_head : 'if' '(' boolean_expression ')' compound_statement;

identifier : T_IDENTIF;
```

### Tree for if
```
left:   condition
mid:    if block
right:  else block
```
### BNF for while loops

while_statement: 'while' '(' true_false_expression ')' compound_statement


### Types of AST Nodes
Type1:
```
Root : Operator
Left : Value | Expression
Right : Value | Expression
```

Type2:
```
Root : Assignment
Left : Value | Expression
Right : SymTab
```

### Comparison Operators
```
> < >= <= != ==
```