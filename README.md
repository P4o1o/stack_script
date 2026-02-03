# sscript - Stack Script Interpreter (Rust Version)

A Rust port of the stack-based programming language interpreter originally written in C.

## Building

```bash
cargo build --release
```

## Running

```bash
# Interactive REPL
cargo run

# With options
cargo run -- -v          # Print top element after each command
cargo run -- -v5         # Print top 5 elements after each command
cargo run -- -m          # Load math library
cargo run -- -s          # Load stack operations library
cargo run -- file.sksp   # Load and execute a file
```

## Testing

```bash
cargo test
```

## Features

### Data Types
- **Integer** - 64-bit signed integers
- **Float** - 64-bit floating point numbers
- **Boolean** - `true` / `false`
- **String** - Double-quoted strings: `"hello"`
- **Instruction** - Quoted code blocks: `[1 2 +]`
- **InnerStack** - First-class stack: `{1 2 3}` or `stack`
- **None** - Null value: `none`
- **Type** - Type values: `INT`, `FLOAT`, `BOOL`, `STR`, `INSTR`, `TYPE`, `NONE`, `STACK`

### Math Operations
| Op | Description | Example |
|----|-------------|---------|
| `+` | Addition | `5 3 +` → `8` |
| `-` | Subtraction | `5 3 -` → `2` |
| `*` | Multiplication | `5 3 *` → `15` |
| `/` | Division | `15 3 /` → `5.0` |
| `%` | Modulo | `17 5 %` → `2` |
| `pow` | Power | `2 10 pow` → `1024.0` |
| `sqrt` | Square root | `9 sqrt` → `3.0` |
| `exp` | Exponential | `1 exp` → `2.718...` |
| `log` | Natural log | `e log` → `1.0` |
| `log2` | Log base 2 | `8 log2` → `3.0` |
| `log10` | Log base 10 | `100 log10` → `2.0` |
| `--` | Negation | `5 --` → `-5` |
| `!` | Factorial | `5 !` → `120.0` |
| `gamma` | Gamma function | `5 gamma` → `24.0` |
| `int` | Convert to int | `3.7 int` → `3` |

### Trigonometric Functions
`sin`, `cos`, `tan`, `arcsin`, `arccos`, `arctan`, `sinh`, `cosh`, `tanh`, `arcsinh`, `arccosh`, `arctanh`

### Boolean Operations
| Op | Description | Example |
|----|-------------|---------|
| `and` | Logical AND | `true false and` → `false` |
| `or` | Logical OR | `true false or` → `true` |
| `xor` | Logical XOR | `true true xor` → `false` |
| `not` | Logical NOT | `true not` → `false` |
| `true` | Push true | |
| `false` | Push false | |

### Comparison Operations
| Op | Description | Example |
|----|-------------|---------|
| `==` | Equal | `5 5 ==` → `true` |
| `!=` | Not equal | `5 3 !=` → `true` |
| `<` | Less than | `3 5 <` → `true` |
| `>` | Greater than | `5 3 >` → `true` |
| `<=` | Less or equal | `5 5 <=` → `true` |
| `>=` | Greater or equal | `5 5 >=` → `true` |

### Stack Operations
| Op | Description | Example |
|----|-------------|---------|
| `dup` / `dup0` | Duplicate top | `42 dup` → `42 42` |
| `dupN` | Duplicate Nth | `1 2 3 dup2` → `1 2 3 1` |
| `dup(expr)` | Dup with expr | `1 2 dup(size 1 -)` → `1 2 1` |
| `swap` / `swap1` | Swap top 2 | `1 2 swap` → `2 1` |
| `swapN` | Swap with Nth | `1 2 3 swap2` → `3 2 1` |
| `swap(expr)` | Swap with expr | |
| `drop` | Remove top | `1 2 drop` → `1` |
| `clear` | Clear stack | |
| `roll` | Roll stack | `1 2 3 roll` → `3 1 2` |
| `top` | Dup bottom | `1 2 3 top` → `1 2 3 1` |
| `digN` | Dig Nth to top | `1 2 3 dig2` → `2 3 1` |
| `dig(expr)` | Dig with expr | |
| `size` | Push stack size | `1 2 3 size` → `1 2 3 3` |
| `empty` | Push if empty | `empty` → `true` |
| `last` | Push if single elem | `42 last` → `42 true` |
| `quote` | Quote value | `42 quote` → `[42]` |
| `compose` | Compose instrs | `[1] [2] compose` → `[1 2]` |
| `apply` | Execute instr | `[1 2 +] apply` → `3` |
| `dip` | Apply under top | `1 2 [+] dip` → `3` |
| `split` | Split instr/str | `[1 2 3] split` → `[1] [2] [3]` |
| `compress` | Stack to inner | `1 2 3 compress` → `{1 2 3}` |

### Inner Stack Operations
| Op | Description | Example |
|----|-------------|---------|
| `stack` | Create empty inner stack | |
| `{...}` | Stack literal | `{1 2 3}` |
| `push` | Push to inner | `stack 1 push` |
| `pop` | Pop from inner | `{1 2} pop` → `{1} 2` |
| `inject` | Execute in inner | `{1 2} [+] inject` → `{3}` |
| `injectN` | Inject in N stacks | |
| `pinjectN` | Parallel inject | |

### Control Flow
| Op | Description | Example |
|----|-------------|---------|
| `if` | Conditional | `true [1] [0] if` → `1` |
| `if(cond)` | Cond from expr | `5 [yes] [no] if(3 >)` |
| `loop` | While-do loop | `1 [1 + dup 5 <] loop` → `5` |
| `loop(cond)` | While loop | `0 [1 +] loop(dup 5 <)` |
| `times(n)` | Repeat N times | `0 [1 +] times(5)` → `5` |
| `try` | Try-catch | `[1 2 +] try` → `3 true` |
| `nop` | No operation | |
| `exit` | Exit program | |

### Type Operations
| Op | Description |
|----|-------------|
| `type` | Push type of top |
| `INT`, `FLOAT`, `BOOL`, `STR`, `INSTR`, `TYPE`, `NONE`, `STACK` | Type constants |

### I/O Operations
| Op | Description |
|----|-------------|
| `print` | Print top |
| `printall` | Print all |
| `load(file)` | Load and execute file |
| `save(file)` | Save stack to file |

### Definition Operations
| Op | Description | Example |
|----|-------------|---------|
| `define(name)` | Define instruction | `[2 *] define(double)` |
| `delete(name)` | Delete definition | `delete(double)` |
| `isdef(name)` | Check if defined | `isdef(double)` → `true/false` |

## Project Structure

```
sscript/
├── Cargo.toml           # Package configuration
├── README.md            # This file
├── src/
│   ├── lib.rs           # Library exports
│   ├── main.rs          # CLI entry point
│   ├── compiler.rs      # Tokenizer and compiler
│   ├── instruction.rs   # Bytecode instruction enum
│   ├── interpreter.rs   # Virtual machine
│   ├── stack.rs         # Stack data structure
│   ├── environment.rs   # User definitions storage
│   ├── error.rs         # Error types
│   ├── primitives.rs    # Constants
│   ├── math_op.rs       # Math operations
│   ├── bool_op.rs       # Boolean operations
│   ├── stack_op.rs      # Stack operations
│   └── types_op.rs      # Type operations
└── tests/
    └── integration_tests.rs  # Comprehensive test suite
```

## Example Programs

### Factorial
```
[dup 1 <= [drop 1] [dup 1 - fact *] if] define(fact)
5 fact  # → 120
```

### Fibonacci
```
[dup dup 1 == swap 0 == or not [dup 1 - fib swap 2 - fib +] [nop] if] define(fib)
10 fib  # → 55
```

### Sum 1 to N
```
[0 swap [dup swap2 + swap 1 + dup swap2 <=] loop drop] define(sumto)
10 sumto  # → 55
```

### GCD
```
[dup 0 == [drop] [dup swap2 swap % gcd] if] define(gcd)
48 18 gcd  # → 6
```

## License

MIT
