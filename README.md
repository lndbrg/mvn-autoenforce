# mvn-autoenforce

## Motivation
Managing dependencies are hard, managing dependencies with multiple versions are even harder and transitive dependencies
might even leak into your project which can lead to unforeseen side effects during runtime (some would call these side
effects bugs 🪳, I call them code spice 🌶).

Most people prefer their code to be without added spice and behave the way that they intended and avoid these unknown
side effects, for maven based projects a good way to do that is using the `RequireUpperBoundDeps` rules of the
`maven-enforcer-plugin`. This plugin usually runs during the `validate` phase of your build lifecycle and if it happens
to stumble upon conflicting dependency versions it gives you a wall of text that mostly just induces eye strain.
It also takes a while to parse this wall of for the human eye and brain, alas the meat machines that we are where
optimized to look at prettier things.

This CLI tool exists to help you parse that wall of text and outputs the topmost dependency version of the problematic
dependency in your pom. The output is in the very new and fancy markup language called XML

## Install
### Requirements
Rust 🦀
### Installation
`cargo install mvn-autoenforce`

## Usage

Run `mvn validate | mvn-autoenforce` and copy the dependencies to your pom.
