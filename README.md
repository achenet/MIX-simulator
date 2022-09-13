# MIX Simulator

## This program is a work in progress

This program emulates the functioning of the MIX computer from Donald Knuth's book _The Art of Computer Programming_.

A byte is either digital, in which case it is has 64 possible values,
or decimal, in which case it has 99 possible values.

A word in memory is 5 bytes plus a sign.

The MIX computer has 4000 memory locations of one word each, indexed 0 to 3999.
It has an overflow toggle, with possible values ON, OFF,
a comparaison operator, with possible values EQUAL, LESS, GREATER,
various IO devices,
and some registers.

The registers are

A - sign plus 5 bytes

X - sign plus 5 bytes

I1, I2, I3, I4, I5, I6 - sign plus 2 bytes

J - 2 bytes, no sign (always positive)

I've yet to find a satisfactory way to handle byte packing,
and so have opted to just assume at that all bytes are composed
of 6 binary bits, capable of representing numbers 0 to 63.
If you have any better ideas, please let me know.
