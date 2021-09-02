///! Universally unique identifiers.
///!
///! The identifiers generated by this module:
///!
///! - are URL safe
///! - contain information on the type of object that they
///!   are identifying
///! - have an extremely low probability of collision
///!
///! Generated identifiers have a fixed length of 32 characters made up
///! of two parts separated by a dot:
///!
///! - 2 characters in the range `[a-z]` that identifying the "family" of
///!   identifiers, usually the type of object the identifier is for
///!   e.g. `fi` = file, `re` = request
///!
///! - 20 characters in the range `[0-9A-Za-z]` that are randomly generated
///!
///! For project identifiers (those starting with 'pr') only lowercase
///! letters are used for compatibility with Docker image naming rules.
///!
///! The total size of the generated ids is 23 bytes which allows it to fit
///! inside a [`SmartString`](https://lib.rs/crates/smartstring) for better
///! performance that a plain old `String`.
///!
///! See
///!  - https://segment.com/blog/a-brief-history-of-the-uuid/
///!  - https://zelark.github.io/nano-id-cc/
///!  - https://gist.github.com/fnky/76f533366f75cf75802c8052b577e2a5
use nanoid::nanoid;
use strum::ToString;

/// The available families of identifiers
#[derive(ToString)]
pub enum Family {
    #[strum(serialize = "no")]
    Node,

    #[strum(serialize = "do")]
    Document,

    #[strum(serialize = "fi")]
    File,

    #[strum(serialize = "pr")]
    Project,

    #[strum(serialize = "se")]
    Session,
}

/// The characters used in the third part of the identifier
const CHARACTERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

// Generate a universally unique identifier
pub fn generate(family: Family) -> String {
    let chars = match family {
        Family::Project => nanoid!(20, &CHARACTERS[..36]),
        _ => nanoid!(20, &CHARACTERS),
    };
    [&family.to_string(), ".", &chars].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use regex::Regex;

    #[test]
    fn test_node_id() -> Result<()> {
        let id = generate(Family::Node);

        assert_eq!(id.len(), 23);

        let re = Regex::new(r"no\.[0-9a-zA-Z]{20}")?;
        assert!(re.is_match(&id));

        Ok(())
    }

    #[test]
    fn test_project_id() -> Result<()> {
        let id = generate(Family::Project);

        assert_eq!(id.len(), 23);

        let re = Regex::new(r"pr\.[0-9a-z]{20}")?;
        assert!(re.is_match(&id));

        Ok(())
    }
}
