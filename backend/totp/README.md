An implementation of https://datatracker.ietf.org/doc/html/rfc6238

# Recommendations

The following are recommendations from the RFC6238 document.

## Time step (period)

It is recommended to use a default time-step size of **30 seconds**. It can be a custom value, but shouldn't be minutes or more.

## Key generation

> **Note:** the `CryptoAlgorithm.generate_key()` method respects the points below.

The length of the key can be any length, but it is recommended to correspond with the corresponing algorithm:
- SHA1: 160 bits (20 characters)
- SHA256: 256 bits (32 characters)
- SHA512: 512 bits (64 characters)

The key should be chosen at random or using a cryptographically strong pseudorandom generator properly seeded with a random value.

## Validation

Tokens are **one time use**, shouldn't be able to use the same OTP twice for the same time.

If there's a time window, an older token shouldn't be able to be used to authenticate if a newer token was used. User will need to wait for the next token.

## Notes

* we also RECOMMEND storing the keys securely in the validation system, and, more specifically, encrypting them using tamper-resistant hardware encryption and exposing them only when required: for example, the key is decrypted when needed to verify an OTP value, and re-encrypted immediately to limit exposure in the RAM to a short period of time

validation
* a validation system SHOULD typically set a policy for an acceptable OTP transmission delay window for validation
* The validation system should compare OTPs not only with the receiving timestamp but also the past timestamps that are within the transmission delay.  A larger acceptable delay window would expose a larger window for attacks. We RECOMMEND that at most one time step is allowed as the network delay.
  * at most one time step is allowed as the network delay
