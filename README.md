# styledtext

A tool help convert nomral ASCII text in English to its stylized versions and vice versa.

## Usage

```
Usage: styledtext [OPTIONS] <TEXT>

Arguments:
  <TEXT>

Options:
      --letter-type <LETTER_TYPE>
          turn ASCII letters into styled letters [default: monospace] [possible values: serif, sansserif, script, fraktur, monospace, doublestruck]
      --letter-style <LETTER_STYLE>
          [default: normal] [possible values: normal, bold, italic, bolditalic]
      --random
          convert with randomly types and styles
      --exclude-types <EXCLUDE_TYPES>
          convert text randomly within given types [possible values: serif, sansserif, script, fraktur, monospace, doublestruck]
      --exclude-styles <EXCLUDE_STYLES>
          convert text randomly within given styles [possible values: normal, bold, italic, bolditalic]
      --ascii
          turn styled letters to ASCII letters
  -h, --help
          Print help
  -V, --version
          Print version
```

## TODO

- [x] Convert ASCII text to styled text
- [ ] Convert styled ASCII text to ASCII text
- [ ] Convert text using random types and styles
