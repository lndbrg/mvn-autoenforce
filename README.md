# mvn-autoenforce

## Motivation
Managing dependencies is hard and transitive dependencies might leak into your project which might have unforeseen
side effects during runtime (some would call them bugs, I call it adding some spiciness to your code). Most people 
do want to avoid the added spiciness of the unknown side effects and for maven based java projects a good way to do that
is using the `RequireUpperBoundDeps` rules of the `maven-enforcer-plugin` plugin. This plugin usually runs during the
`validate` phase of your build lifecycle and if it happens to stumble upon conflicting dependency versions it gives you
a wall of text that mostly just induces eye strain and takes a while to parse for the human eye and brain.

This CLI tool exists to help you parse that wall of text and outputs the topmost dependency version of the problematic
dependency in your pom. The output is in the very new and fancy markup language called XML

## Install
### Requirements
Rust 🦀
### Installation
`cargo install mvn-autoenforce`

## Usage

Run `mvn validate | mvn-autoenforce` and copy the dependencies to your pom.
