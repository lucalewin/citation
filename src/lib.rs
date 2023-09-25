use std::path::PathBuf;

/// https://github.com/citation-file-format/citation-file-format/blob/main/schema-guide.md#valid-keys
#[allow(unused)]
pub struct Citation {
    /// A description of the software or dataset.
    /// required: false
    r#abstract: Option<String>,

    /// The authors of a software or dataset
    /// required: true (at least one item in the Vec)
    authors: Vec<Author>,

    /// The Citation File Format schema version that the `CITATION.cff` file adheres to for providing the citation metadata.
    /// required: true
    cff_version: String,

    /// The commit hash or revision number of the software version.
    /// required: false
    commit: Option<String>,

    /// required: false
    contact: Option<Contact>, // FIXME

    /// The date the software or data set has been released. Format is 4-digit year, 2-digit month, 2-digit day of month, separated by dashes.
    /// required: false
    date_released: Option<String>,

    /// required: false
    dio: Option<String>, // FIXME

    /// The identifiers of the software or dataset.
    /// required: false
    identifiers: Vec<Identifier>,

    /// Keywords that describe the work.
    /// required: false
    keywords: Vec<String>,

    /// The [SPDX license identifier(s)](https://spdx.dev/ids/) for the license(s) under which the work is made available. When there are multiple licenses, it is assumed their relationship is OR, not AND.
    /// required: false
    license: Option<License>,

    /// The URL of the license text under which the software or dataset is licensed (only for non-standard licenses not included in the SPDX License List).
    /// required: false
    license_url: Option<String>,

    /// A message to the human reader of the `CITATION.cff` file to let them know what to do with the citation metadata.
    /// 
    /// default: `If you use this software, please cite it using the metadata from this file.`  
    /// 
    /// required: true
    message: String,

    /// A reference to another work that should be cited instead of the software or dataset itself. Note that the principles of [software citation](https://doi.org/10.7717/peerj-cs.86) and [data citation](https://doi.org/10.25490/a97f-egyk) require that software should be cited on the same basis as any other research product such as a paper or a book. Adding a different preferred citation may result in a violation of the respective primary principle, "Importance", when others cite this work.
    /// 
    /// required: false
    preferred_citation: Option<String>,

    /// Reference(s) to other creative works. Similar to a list of references in a paper, references of the software or dataset may include other software (dependencies), or other research products that the software or dataset builds on, but not work describing the software or dataset.
    /// 
    /// required: false
    references: Vec<Reference>,

    /// The URL of the software or dataset in a repository/archive (when the repository is neither a source code repository nor a build artifact repository).
    /// 
    /// required: false
    repository: Option<String>,

    /// The URL of the work in a build artifact/binary repository (when the work is software).
    /// 
    /// required: false
    repository_artifact: Option<String>,

    /// The URL of the work in a source code repository.
    /// 
    /// required: false
    repository_code: Option<String>,

    /// The name of the software or dataset.
    /// 
    /// required: true
    title: String,

    /// The type of the work that is being described by this CITATION.cff file.
    /// 
    /// default: `Software`
    /// 
    /// required: false
    r#type: Option<Type>,

    /// The URL of a landing page/website for the software or dataset.
    /// 
    /// required: false
    url: Option<String>,

    /// The version of the software or dataset.
    /// 
    /// required: false
    version: Option<String>
}

impl Citation {
    pub fn read(path: PathBuf) -> std::io::Result<Self> {
        todo!()
    }

    pub fn validate(&self) -> Result<(), ()> {
        todo!()
    }
}

pub enum Author {
    Person {
        given_names: String,
        family_names: String,
        email: Option<String>,
        dio: Option<String>
    },
    Entity { name: String }
}

pub struct Contact {}

pub struct Identifier {}

pub struct Reference {}

pub enum License {}

#[derive(Default)]
pub enum Type {
    #[default]
    Software,
    Dataset
}