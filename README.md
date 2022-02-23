# `horned-visit` [![Star me](https://img.shields.io/github/stars/fastobo/horned-visit.svg?style=social&label=Star&maxAge=3600)](https://github.com/fastobo/horned-visit/stargazers)

*Visitor traits for [`horned-owl`](https://github.com/phillord/horned-owl) with overloadable implementations.*

[![Actions](https://img.shields.io/github/workflow/status/fastobo/horned-visit/Test?style=flat-square&maxAge=600)](https://github.com/fastobo/horned-visit/actions)
[![Codecov](https://img.shields.io/codecov/c/gh/fastobo/horned-visit/master.svg?style=flat-square&maxAge=600)](https://codecov.io/gh/fastobo/horned-visit)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/horned-visit)
[![Crate](https://img.shields.io/crates/v/horned-visit.svg?maxAge=600&style=flat-square)](https://crates.io/crates/horned-visit)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/horned-visit)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/fastobo/horned-visit/blob/master/CHANGELOG.md)
[![GitHub issues](https://img.shields.io/github/issues/fastobo/horned-visit.svg?style=flat-square)](https://github.com/fastobo/horned-visit/issues)


## 🗺️ Overview

This library provides visitor traits for the [`horned-owl`](https://crates.io/crates/horned-owl)
object model, which can be used to easily implement algorithms to query or
edit an ontology.

## 🔌 Usage

Add the latest versions of `horned-owl` and `horned-visit` to the
`[dependencies]` sections of your `Cargo.toml` manifest:
```toml
[dependencies]
horned-owl = "0.11.0"
horned-visit = "0.1.0"
```

Then use the `horned_visit::Visit` or `horned_visit::VisitMut` traits to
implement an algorithm. The `horned_visit::visit` and `horned_visit::visit_mut`
modules contain default methods implementations.

## 💡 Example

OWL2 does not require all entities to be declared (see the
[specification](https://www.w3.org/TR/owl2-syntax/#Declaration_Consistency)),
but it can be required by convention to help catching typos. Here is how
an algorithm could be implemented with `horned-visit` that ensures that
all the IRI referencing OWL2 classes are declared in an ontology document:

```rust
extern crate horned_owl;
extern crate horned_visit;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use horned_owl::model::*;
use horned_visit::Visit;

#[derive(Default, Debug)]
struct ClassDeclarationChecker<'ast> {
    declared: HashSet<&'ast IRI>,
    used: HashSet<&'ast IRI>
}

impl<'ast> Visit<'ast> for ClassDeclarationChecker<'ast> {
    fn visit_declare_class(&mut self, declare_class: &'ast DeclareClass) {
        self.declared.insert(&declare_class.0.0);
        horned_visit::visit::visit_declare_class(self, declare_class);
    }
    fn visit_class(&mut self, class: &'ast Class) {
        self.used.insert(&class.0);
    }
}

pub fn classes_well_declared(ontology: &horned_owl::ontology::set::SetOntology) -> bool {
    let mut checker = ClassDeclarationChecker::default();
    ontology.iter().for_each(|aa| checker.visit_annotated_axiom(aa));
    checker.used.is_subset(&checker.declared)
}

let mut f = File::open("tests/data/bfo.owl").map(BufReader::new).unwrap();
let ontology = horned_owl::io::rdf::reader::read(&mut f).unwrap().0.into();
assert!(classes_well_declared(&ontology));
```

## 💭 Feedback

### ⚠️ Issue Tracker

Found a bug ? Have an enhancement request ? Head over to the
[GitHub issue tracker](https://github.com/fastobo/horned-visit/issues) of the project if
you need to report or ask something. If you are filling in on a bug, please include as much
information as you can about the issue, and try to recreate the same bug in a simple, easily
reproducible situation.

## 📋 Changelog

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
and provides a [changelog](https://github.com/althonos/horned-visit/blob/master/CHANGELOG.md)
in the [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) format.

## 📜 License

This library is provided under the open-source
[MIT license](https://choosealicense.com/licenses/mit/).

## 📰 Citation

*This project was developed by [Martin Larralde](https://github.com/althonos/)
during his PhD project at the [European Molecular Biology Laboratory](https://www.embl.de/)
in the [Zeller team](https://github.com/zellerlab).*
