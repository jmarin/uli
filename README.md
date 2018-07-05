# uli
Universal Loan Identifier (ULI) validation and check digit generation for the Home Mortgage Disclosure Act (HMDA). Procedure for generating and validating described [here](https://www.consumerfinance.gov/eregulations/1003-C/2015-26607_20200101#1003-C-1<Paste>)

## Developing


## Using

To validate a ULI: 

```shell
uli validate <uli>
```

To generate a check digit:
```shell
uli check-digit <loan_id>
```

