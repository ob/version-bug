use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use version_compare::{Cmp, Part, Version};

fn main() -> io::Result<()> {
    let path = Path::new("versions.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }

    let mut versions: Vec<Version> = Vec::new();
    for line in &lines {
        match Version::from(line.as_str()) {
            Some(version) => versions.push(version),
            None => eprintln!("Invalid version format: {}", line),
        }
    }

    // Test total order properties:
    test_reflexivity(&versions);
    test_antisymmetry(&versions);
    test_transitivity(&versions);

    // this panics
    versions.sort_by(|a, b| compare_versions(a, b).ord().unwrap());

    for version in versions {
        println!("{}", version);
    }

    Ok(())
}

pub fn test_reflexivity(versions: &Vec<Version>) {
    for version in versions {
        let cmp = version.compare(version);
        assert_eq!(
            cmp,
            Cmp::Eq,
            "Reflexivity failed for version: {}. Expected Cmp::Eq but got {:?}",
            version,
            cmp
        );
    }
}

pub fn test_antisymmetry(versions: &Vec<Version>) {
    for a in versions {
        for b in versions {
            let ab = a.compare(b);
            let ba = b.compare(a);
            match ab {
                Cmp::Eq => {
                    assert_eq!(
                        ba,
                        Cmp::Eq,
                        "Antisymmetry failed for a: {}, b: {} (a.compare(b) == Eq but b.compare(a) == {:?})",
                        a,
                        b,
                        ba
                    );
                }
                Cmp::Lt => {
                    assert_eq!(
                        ba,
                        Cmp::Gt,
                        "Antisymmetry failed for a: {}, b: {} (a.compare(b) == Lt but b.compare(a) == {:?})",
                        a,
                        b,
                        ba
                    );
                }
                Cmp::Gt => {
                    assert_eq!(
                        ba,
                        Cmp::Lt,
                        "Antisymmetry failed for a: {}, b: {} (a.compare(b) == Gt but b.compare(a) == {:?})",
                        a,
                        b,
                        ba
                    );
                }
                // If you ever get one of these, consider it an error.
                unexpected => panic!(
                    "Unexpected variant {:?} from a.compare(b) for a: {}, b: {}",
                    unexpected, a, b
                ),
            }
        }
    }
}

pub fn test_transitivity(versions: &Vec<Version>) {
    for a in versions {
        for b in versions {
            for c in versions {
                let ab = a.compare(b);
                let bc = b.compare(c);
                let ac = a.compare(c);

                if ab == Cmp::Eq && bc == Cmp::Eq {
                    assert_eq!(
                        ac,
                        Cmp::Eq,
                        "Transitivity (equality) failed for a: {}, b: {}, c: {} (expected Eq, got {:?})",
                        a,
                        b,
                        c,
                        ac
                    );
                }
                if ab == Cmp::Lt && bc == Cmp::Lt {
                    assert_eq!(
                        ac,
                        Cmp::Lt,
                        "Transitivity failed for a: {}, b: {}, c: {} (expected a < c, got {:?})",
                        a,
                        b,
                        c,
                        ac
                    );
                }
                if ab == Cmp::Gt && bc == Cmp::Gt {
                    assert_eq!(
                        ac,
                        Cmp::Gt,
                        "Transitivity failed for a: {}, b: {}, c: {} (expected a > c, got {:?})",
                        a,
                        b,
                        c,
                        ac
                    );
                }
            }
        }
    }
}

/////
fn compare_part(a: &Part, b: &Part) -> Cmp {
    match (a, b) {
        (Part::Number(x), Part::Number(y)) => {
            if x < y {
                Cmp::Lt
            } else if x > y {
                Cmp::Gt
            } else {
                Cmp::Eq
            }
        }
        (Part::Text(x), Part::Text(y)) => {
            let x_lower = x.to_lowercase();
            let y_lower = y.to_lowercase();
            if x_lower < y_lower {
                Cmp::Lt
            } else if x_lower > y_lower {
                Cmp::Gt
            } else {
                Cmp::Eq
            }
        }
        (Part::Number(_), Part::Text(_)) => Cmp::Lt,
        (Part::Text(_), Part::Number(_)) => Cmp::Gt,
    }
}

fn is_trivial(part: &Part) -> bool {
    match part {
        Part::Number(n) => *n == 0,
        Part::Text(s) => s.is_empty(),
    }
}

pub fn compare_versions<'a>(a: &Version<'a>, b: &Version<'a>) -> Cmp {
    let mut iter_a = a.parts().iter();
    let mut iter_b = b.parts().iter();

    loop {
        match (iter_a.next(), iter_b.next()) {
            (Some(part_a), Some(part_b)) => {
                let cmp = compare_part(part_a, part_b);
                if cmp != Cmp::Eq {
                    return cmp;
                }
            }
            (Some(part_a), None) => {
                // There are remaining parts in `a`. If any of these parts are non-trivial,
                // then `a` is greater; otherwise, the versions are equal.
                if iter_a
                    .chain(std::iter::once(part_a))
                    .any(|p| !is_trivial(p))
                {
                    return Cmp::Gt;
                } else {
                    return Cmp::Eq;
                }
            }
            (None, Some(part_b)) => {
                // There are remaining parts in `b`. If any of these parts are non-trivial,
                // then `a` is less than `b`; otherwise, they are equal.
                if iter_b
                    .chain(std::iter::once(part_b))
                    .any(|p| !is_trivial(p))
                {
                    return Cmp::Lt;
                } else {
                    return Cmp::Eq;
                }
            }
            (None, None) => return Cmp::Eq,
        }
    }
}
