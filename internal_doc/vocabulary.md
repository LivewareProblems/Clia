# Vocabulary reference

Clia is sitting at the confluence of multiple domains. Programming language, parsing, security, ergonomics, CLI, packaging,scripting, ...

As such, it is not rare that a word is used to define different things in these domains, or that some concepts that need to be reified in discussions around Clia have no name in the litterature. This is why this vocabulary reference exists.

This is a live document. It will evolve and change, like the language of the people working with and on Clia. This is not a strong prescriptive document, more a useful representation of the language used by the people working on, around or with Clia.

## Product

A product is the programs the users of Clia are building that are suposed to be called from the CLI by users. This is opposed to libraries.

We do not use the terminology of "tools" or "application" for end products. Tools is used already for the collection of elements that form the Clia environment used to help author code. Compiler, formatter, test runner, package manager, etc.

Application have a "long running" expectation , which does not correspond to the use case Clia target.

## Server

A server is an program that run for a long time, as a daemon, to answer a variety of requests. This is not a domain that Clia targets.
