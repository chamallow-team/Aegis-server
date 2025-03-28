# Versions

## Specifying versions

| Format                | Example                    | In which case                                                                                                                                                                                                                 |
|-----------------------|----------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `<dev>:<version>`     | `satellites:6.8.2`         | When you want to specify a simple dependency. You can also specify less numbers, such as `satellites:6.8` to target all version that matches.  You can also add a `=` at the **start** of the version, but it is not required |
| `<dep>:<v1><op><v2>`  | `satellites:6.1 - 6.8`     | Specify a range in which the versions are accepted. That way, you can for example target the version `6.1` and thus until the version `6.8`, where the next version includes a breaking (or unwanted) change                  |
| `<dep>:<op><version>` | `satellites:>6.2.7`        | When used, it'll specify that it accepts any version accordingly to the operator. The allowed operators are `>`, `<`, `<=` and `>=`                                                                                           |
| `<dep>:<a><bin><b>`   | `satellites:>6.7 && <=7.1` | Allows binary operations  (`\|\|` and `&&`) for dependencies. In the example, it takes all versions superior to `6.7` but less or equal to `7.1`.                                                                             |
