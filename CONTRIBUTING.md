# How To Contribute

First of all, thanks for using LibRunner! It would be great to learn more about your experience. If you could share some feedback about the documentation, the examples, the readability, and even the code, you would help us to make it better. If you want to go a step further, you could make improvements yourself and submit pull requests. 

This document is a guide to learn how to contribute to LibRunner. You can contribute by:

* Writing and improving the documentation
* Creating more code examples
* Reporting bugs
* Requesting new features
* Writing code

## Publishing a New Version

1. Run all tests to make sure everything is still working as expected:

       $ cargo test

2. Generate the documentation at `target/doc/librunner` to check if it was properly updated with the latest changes:

       $ cargo doc

2. Increment the version number in the file `Cargo.toml`:

       version = "0.3.0" -> version = "0.4.0"

2. Commit the file `Cargo.toml` and push to the repository:

       $ git add Cargo.toml
       $ git commit -m "Incremented version number"
       $ git push origin main

3. Check if everything is ready to publish the library:

       $ cargo publish --dry-run

4. Finally, publish the library:

       $ cargo publish