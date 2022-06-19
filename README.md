Grammar :

<json> ::= <object> | <array> | <number> | <string> | <boolean> | <null>

<object> ::= '{' [ <member> *(', ' <member>) ] '}'

<member> ::= <string> ': ' <json>

<array> ::= '[' [ <json> *(', ' <json>) ] ']'
