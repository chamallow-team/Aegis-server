use crate::error::PackError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the header of a pack, with its metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackHeader {
    /// The mod ID is necessary for the game (especially referring to the mod in-game), for the mod marketplace and for other mods to specify dependencies.
    ///
    /// The ID must be made of alphanumerical characters and of _ or -, and must be longer than 5 characters and lower than 64 characters. You must also be sure that the ID is unique.
    mod_id: String,

    /// The name of the mod, as displayed to the user
    name: String,
    /// Translations for the name of the mod
    #[serde(default)]
    name_translations: HashMap<String, String>,

    /// The description of the mod, as displayed to the user
    description: String,
    /// Translations for the description of the mod
    #[serde(default)]
    description_translations: HashMap<String, String>,

    /// The version of the mod
    ///
    /// The version field has two main usages:
    ///
    /// 1. It'll allow the mod manager/marketplace to propose different versions
    /// 2. Gives to the server the right mod with the right contents, using the version.
    ///
    /// Please note that the recommended way of versioning your mod is to use semantics, where a version is:
    /// `<MAJOR>:<MINOR>:<PATCH>`
    ///
    /// You can also use tags such as `*-alpha` or even `*-snapshot`!
    ///
    /// For more details, you can read the documentation or read this article
    ///
    /// This field is required at all times
    version: semver::Version,
    /// The authors is a list of names/emails for the author(s) and contributor(s).
    /// The value must be a list and can be simply a name, such as `Aegis Team`, or a complex name, following the git (or if you want, the email system).
    ///
    /// The git way of writing an author is:
    ///
    /// `name <email>`
    ///
    /// Which can be expressed, technically, with the following regex:
    ///
    /// ```regex
    /// ^[^<]+(<email_regex>)?
    /// ```
    ///
    /// The email verification has been stripped down for consistency and documentation readability
    ///
    /// This field is required to be published publicly

    #[serde(default)]
    authors: Vec<String>,

    // TODO add dependencies parsing
    #[serde(default)]
    dependencies: Vec<String>,

    // TODO add compatible game versions parsing
    #[serde(default)]
    compatible_with: Vec<String>,

    /// The homepage of the mod
    ///
    /// This field is not required at all, and can redirect to the mod web page.
    ///
    #[serde(default)]
    homepage: Option<String>,
    /// The repository of the mod
    ///
    /// Same as the homepage, the `repository` field is not required and can be used to specify the `git repositery` (you can use GitHub, Gitlab, Bitbucket or any version control software).
    #[serde(default)]
    repository: Option<String>,
    /// The license of the mod
    #[serde(default)]
    license: Option<String>,
}

impl PackHeader {
    /// Parse a pack header from a buffer
    ///
    /// # Errors
    ///
    /// This function will return an error if the buffer is not a valid msgpack file, or if the mod ID is invalid.

    pub(crate) fn parse_from_buf(buf: &[u8]) -> Result<PackHeader, PackError> {
        let header: PackHeader = rmp_serde::from_slice(buf).map_err(PackError::from)?;

        // Test that the mod ID is valid
        if !crate::MOD_ID_REGEX.is_match(&header.mod_id) {
            return Err(PackError::InvalidModID);
        }

        Ok(header)
    }
}
