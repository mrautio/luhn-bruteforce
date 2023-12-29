![Build](https://github.com/mrautio/luhn-bruteforce/workflows/Build/badge.svg)

# Luhn bruteforce

Tool to deduce or generate various values that utilize [Luhn checksum (modulus 10)](https://en.wikipedia.org/wiki/Luhn_algorithm), such as, credit card numbers (PANs).

## Examples

### Deducing list of values when checksum is known

This prints values with correct luhn checksum.
`--values` parameter takes multiple arguments.
- Comma (`,`) separates a permutation option
- Space (` `) separates a permutation section of the value
- `N` character is a shorthand for "any digit" permutation `0,1,2,3,4,5,6,7,8,9`

```
./luhn-brute --values 411111,400000 N 0,1,3,8,9 11111111
4111110311111111
4111111111111111
4111112911111111
4111116011111111
4111117811111111
4000000011111111
4000001811111111
4000005911111111
4000008311111111
4000009111111111
```

### Validate if value has a correct checksum

Program exits successfully (retval 0) if one or more valid checksums are encountered

```
./luhn-brute --values 4111111111111111
4111111111111111
```

### Generate a checksum for a value

Checksum is generated and added to the suffix for the value

```
./luhn-brute --values 444444444444444 --generate-luhn
4444444444444448
```
