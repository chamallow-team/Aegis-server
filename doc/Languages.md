# Languages

As the game is made to be played online by a lot of people, it's a necessity to specify the way languages will be
processed.

## Identifying a language

In the documentation, you'll find languages such as `fr-CA`, `pt_PT`, `zh` and so on. These are **Unicode language
identifiers**.

We use the [**IETF BCP 47**](https://en.wikipedia.org/wiki/IETF_language_tag) language tag.

> To distinguish language variants for countries, regions, or writing systems (scripts), IETF language tags combine
> subtag from other standards such as _ISO 639_, _ISO 15924_, _ISO 3166-1_ and _UN M.49_. For example, the tag `en`
> stands for English; `es-419` for Latin American Spanish; `rm-sursilv` for Romansh Sursilvan; `sr-Cyrl` for Serbian
> written in Cyrillic script; `nan-Hant-TW` for Min Nan Chinese using traditional Han characters, as spoken in Taiwan;
> `yue-Hant-HK` for Cantonese using traditional Han characters, as spoken in Hong Kong; and `gsw-u-sd-chzh` for Zürich
> German.


This standardized code ensures simple implementation and seamless integration with user systems like web browsers and
operating systems.

## Testing a code

If you want to test whether a code is good or not, you can
use [Unicode Utilities: Unicode Language Identifiers and BCP47](https://util.unicode.org/UnicodeJsps/languageid.jsp?l=en).