target remote :2331

# By default, GDB wants to page output so it doesn't flood.
# We don't want that, so set it to off
# Not necessary in batch mode
set pagination off

# print demangled symbols
set print asm-demangle on

set confirm off

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# in case the debugger didn't halt on connect, we should do that now
monitor halt
monitor reset 
load
monitor go
quit