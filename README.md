# words_cli

## info

a cli tool and wrapper around [ispell](https://github.com/lise-henry/rust-ispell) and [dictionaryapi](https://dictionaryapi.dev/) to provide spelling suggestions and word definitions respectively

## usage

the `-s` or `--suggest` option will print a list of spelling suggestions

```
words_cli -s flgrent

#> flagrant
#> fragrant
#> flagrancy
#> flagrantly
#> filigreed
#> flagrance
#> filigreeing
#> flagellant
#> belligerent
```

the `-d` or `--definition` option will look for a cached definition or fetch from https://api.dictionaryapi.dev see https://dictionaryapi.dev/ for the website

```
words_cli -d flagrant

#> flagrant
#>   adjective
#>     (of something considered wrong or immoral) conspicuously or obviously offensive.
#>
#>     example
#>       a flagrant violation of the law
#>
#>     synonyms
#>       blatant
#>       glaring
#>       obvious
#>       overt
#>       evident
#>       conspicuous
```

### tips

select from the suggestion list with fzf

`words_cli -s flagrnt | fzf`


use stdin instead of giving a query on the cli

`echo 'flagrnt' | words_cli -s -`

## help

```
words_cli
a tool for words

NOTE: you can specify stdin by giving a -

USAGE:
    words_cli [OPTIONS]

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -c, --columns <columns>
            columns to align definition text

            this will make the definition text stay within the specified columns
    -d, --define <define>
            print word definition

    -s, --suggest <suggest>
            print word suggestions
```
