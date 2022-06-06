# Smart BMI Contract

Near impl of Body Mass Index (BMI) of person derrived from weight and height.

BMI helps measure obesity rate which helps to evaluate obesity level in people. 

Project structure for writing smart contracts in Rust for NEAR Protocol

# Required Software

- Rust 1.60.0 + cargo
- Node.js v12.21.0
- NEAR CLI 3.2.0

## Overview

let's compute someones random body mass index 

``` near call bmi.kherld.testnet compute '{"weight":52,"height":127.0,"permit":true}'  --accountId random.testnet ```

Set ``` "permit":true ``` to save your data 

This will result to

 - Log [bmi.kherld.testnet]: random.testnet You are Obese  
 - Log [bmi.kherld.testnet]: BMI: 32
 - Log [bmi.kherld.testnet]: Permission Accepted
 - Log [bmi.kherld.testnet]: BIOSECURITY MEASURES ARE IN EFFECT

 
 ## Author

👤 **Author**

- GitHub:  [@kherld-hussein](https://github.com/kherld-hussein)
- Twitter: [@kherld-hussein](https://twitter.com/kherldhussein)

# BMI