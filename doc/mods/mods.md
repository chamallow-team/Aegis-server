`version 0.0.1`

# Mods documentation

## Usage

This documentation will describe how a mod is intended to be defined and built.

## Structure

A _mod_ can have the following structure:

```
.
├── entities
│   ├── a6a0a341-a46f-4e5c-bd28-0ad5fb946331.mp
│   ├── c7ef1301-cee1-4dc5-bad5-22ea8a23dbd3.mp
│   └── caa42652-ed69-4176-a880-28c3ab01c7ad.mp
├── maps
│   ├── athena.map
│   ├── world_2050.map
│   └── world_2025.map
├── entities.indexes
├── research_tree.mp
├── groups.mp
└── config.mp
```

[Generated using: https://www.text-2-tree.com/]:#

> ℹ️ All files with the extension `.mp` are files that are stored using the [msgpack](https://msgpack.org/) format (more
> efficient JSON serialization).

As for which files are required or not, please note that the file `config.mp` is **ESSENTIAL**, as it contains all
information related to the mod.

Other files importance/requirements will be specified in their directories accordingly.

## Header / mod config

> Per `header`, we mean the `config.mp` file. Please take a look at [Structure](#structure)

Are contained the following data:

```json
{
  "mod_id": "french_air_space_forces",
  "name": "French Air and Space Forces",
  "name_translations": {
    "fr": "Forces aériennes et spatiales françaises",
    "zh": "法国航空航天部队"
  },
  "description": "Adds entities, groups, and technologies related to the French Air and Space Forces.",
  "description_translations": {
    "fr": "Ce mod ajoute des entités, des groupes et des technologies liés à l'armée de l'air et de l'espace française.",
    "zh": "此模组添加了与法国空军和航天军相关的实体、团体和技术。"
  },
  "version": "1.0.0",
  "authors": [
    "Aegis Team",
    "Contributor Name"
  ],
  "dependencies": [
    "base_game",
    "other_mod:1.2.3"
  ],
  "compatible_with": [
    "1.0.0 - 1.2"
  ],
  "homepage": "https://example.com/french_air_space_forces",
  "repository": "https://github.com/aegis-team/french_air_space_forces",
  "license": "MIT"
}
```

[Generated using: https://www.text-2-tree.com/]:#

_This is an example file for a hypothetical mod called "French air force"._

### Mod ID

The mod ID is necessary for the game (especially referring to the mod in-game), for the mod marketplace and for other
mods to specify dependencies.

The ID must be made of **alphanumerical characters** and of `_` or `-`, and must be longer than 5 characters and lower
than 64 characters.
You must also be sure that the ID is unique.

In a technical aspect (specially for developers), the ID must verify the following pattern:

```regex
^[a-zA-Z0-9_-]{6,63}$
```

_We replaced `\w` with `a-zA-Z0-9_` for better understanding of what characters are allowed or not, as it's
an [aliases](https://www.w3schools.com/jsref/jsref_regexp_wordchar.asp)_

> ⚠️ The ID `base_game` and all IDs starting with `devs_xxxx` cannot be used, as these are used to mention the base game
> or mods developed by the development team.

### Name and description (with translations)

> For language codes, please see [Identifying a language](../Languages.md#identifying-a-language)

The field `name` is necessary and will be used as a display for users.
The main difference with `description` is that the description is not necessary.

For the two of them, they can have `?_localizations` field each with _language codes_ and the following translation.

For example, if a user's system locale is Canadian French (`fr-CA`), and the mod provides translations for `fr-CA`,
`fr`, and `en`, the `fr-CA` translation will be displayed.
If `fr-CA` is not available, it will fall back to `fr`, and then `en`.
If none of these are found, the default `name` or `description` will be used.

### Version

> ⚠️ This field is required at all times

The version field has two main usages:

1. It'll allow the mod manager/marketplace to propose different versions
2. Gives to the server the right mod with the right contents, using the version.

> Please note that the recommended way of versioning your mod is to use [semantics](https://semver.org/), where a
> version is:
> ``<MAJOR>:<MINOR>:<PATCH>``
>
> You can also use **tags** such as `*-alpha` or even `*-snapshot` !
>
> For more details, you can read the [documentation](https://semver.org/) or
> read [this article](https://www.baeldung.com/cs/semantic-versioning)

### Authors

> ⚠️ This field is required to be published publicly

The authors is a list of names/emails for the author(s) and contributor(s).
The value must be a list and can be simply a name, such as `Aegis Team`, or a complex name, following the git (or if you
want, the email system).

The _git way_ of writing an author is:

```
name <email>
```

Which can be expressed, technically, with the following regex:

```regex
^[^<]+(<email_regex>)?
```

_The email verification has been stripped down for consistency and documentation readability_

> <details>
>     <summary>Validating an email (don't open if you fear regexes)</summary>
> For validating the email, the following regex can be used:
>
> ```regex
> ^([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22))*\x40([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d))*$
> ``` 
> </details>

### Dependencies

As said for the [mod ID](#mod-id), it is used also to specify dependencies.
Dependencies can be mentioned in different ways,
as explained in [Specifying versions](../Versions.md#specifying-versions)

To give an example:

```json
{
  "dependencies": [
    "base_game:1",
    "satellites:6.8.2",
    "satellites:6.1 - 6.8",
    "satellites:>6.2.7",
    "virus_deceases:>6.7 && <=7.1"
  ]
}
```

_Specifying `base_game@1` means that the mod is made for the `v1` of the game, and that default assets of the non-modded
games are required.
Therefore, you cannot use this mod if you disabled the base game assets._

### Homepage

The field `homepage` is not required at all, and can redirect to the mod web page.

### Repository

Same as the [homepage](#homepage), the `repositery` field is not required and can be used to specify the
`git repositery` (you can use GitHub, Gitlab, Bitbucket or any version control software).

### Licence

The license is not required but recommended (a lot) if the source code is available and if you don't want to have your
mod stolen.

You can have a list of _all open-source
licenses_ [here (wikipedia.org)](https://en.wikipedia.org/wiki/Comparison_of_free_and_open-source_software_licenses)

---

## Data

Now that we've gone through all the config file specifications, we can take a look at the different parts of the mod.

Please note that all these parts are not required for the mod to work (when there will be exceptions, it'll be written
down).

### Entities

### Research tree

### Maps

