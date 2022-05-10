# Calculator

This is a basic cpi app for me to learn rust, as I have with other languages in the past.

I am planning on building an AST and parser

## CLI Ideas

`calc [options]` - REPL
`calc [options] <text>` - evaluate text

### Options

Unknown, none for now. ~~Maybe help?~~ **Not yet**

## Syntax

### Stage 1: *basic math*

|         Tokens        |   Operation   | Character |
|          ---          |      ---      |    ---    |
| Plus                  |  Adding       |    `+`    |
| Minus                 |  Subtracting  |    `-`    |
| Asterisk              |  Multiplying  |    `*`    |
| Slash                 |  Dividing     |    `/`    |
| Open Parenthesis      |  OOO*         |    `(`    |
| Close Parenthesis     |  OOO*         |    `)`    |

\* OOO - Order of Operations

#### Stage 1 AST

Basic math AST I guess

The multiply/divide must take presidence over adding/subtracting

		"3 * (4 - 2)"	 vs 	"3 * 4 - 2"

			*							-
		   / \						   / \
		  3   -						  *   2
		     / \					 / \
			4   2					3   4

Stage 2:

	DO NOT EXPAND ON THIS BEFORE FINISHING THE ABOVE.