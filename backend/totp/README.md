An implementation of https://datatracker.ietf.org/doc/html/rfc6238

# Recommendations

The following are recommendations from the RFC6238 document.

## Time step (period)

It is recommended to use a default time-step size of **30 seconds**. It can be a custom value, but shouldn't be minutes or more.

## Key generation

> **Note:** the `CryptoAlgorithm.generate_key()` method respects two of the three points below.

The length of the key can be any length, but it is recommended to correspond with the corresponding algorithm:
- SHA1: 160 bits (20 characters)
- SHA256: 256 bits (32 characters)
- SHA512: 512 bits (64 characters)

The key should be chosen at random or using a cryptographically strong pseudorandom generator properly seeded with a random value.

The key should be encrypted using a strong algorithm, and the encryption key should be stored in a secure location and only accessible when needed.

## Cleanup

> **Note:** Whenever the `Totp` struct is dropped/out of scope, the secret is zeroized out, which respects this point.

Data should be cleaned up and have short lifespan, for secret keys. Limiting the time it is available in memory.

## Validation

Tokens are **one time use**, shouldn't be able to use the same OTP twice for the same time.

If there's a time window, an older token shouldn't be able to be used to authenticate if a newer token was used. User will need to wait for the next token.

It is also recommended that at most one time step is allowed as the network delay. This should be configurable.
