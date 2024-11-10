An implementation of https://www.rfc-editor.org/rfc/rfc6238

## Notes

* keys SHOULD be chosen at random or using a cryptographically strong pseudorandom generator properly seeded with a random value
  * https://www.rfc-editor.org/rfc/rfc4086
* keys SHOULD be of the length of the HMAC output to facilitate interoperability
* we also RECOMMEND storing the keys securely in the validation system, and, more specifically, encrypting them using tamper-resistant hardware encryption and exposing them only when required: for example, the key is decrypted when needed to verify an OTP value, and re-encrypted immediately to limit exposure in the RAM to a short period of time
* we RECOMMEND a default time-step size of 30 seconds

validation
* a validation system SHOULD typically set a policy for an acceptable OTP transmission delay window for validation
* The validation system should compare OTPs not only with the receiving timestamp but also the past timestamps that are within the transmission delay.  A larger acceptable delay window would expose a larger window for attacks. We RECOMMEND that at most one time step is allowed as the network delay.
  * at most one time step is allowed as the network delay

* shouldn't be able to use the same OTP twice for the same time. one time use
* if there's a time window, an older token shouldn't be able to be used to authenticate if a newer token was used. user will need to wait for the next token