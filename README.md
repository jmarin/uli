# uli
Universal Loan Identifier (ULI) validation and check digit generation for the Home Mortgage Disclosure Act (HMDA). Procedure for generating and validating described [here](https://www.consumerfinance.gov/eregulations/1003-C/2015-26607_20200101#1003-C-1<Paste>)

## Installing

If you have a `rust` development environment, from the root directory of the project: 

```shell
cargo build --release
cargo install --path .
```

If you are on Mac OS X:

```shell
brew tap jmarin/uli
```

```shell
brew install jmarin/uli/uli
```

## Using

To validate a ULI: 

```shell
uli --validate 10Bx939c5543TqA1144M999143X38
```

The output of this command should be `ULI 10Bx939c5543TqA1144M999143X38 is valid`

To generate a check digit (COMING SOON):
```shell
uli --check-digit <loan_id>
```

