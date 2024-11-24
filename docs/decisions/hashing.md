All passwords, verification codes, API tokens, are hashed using Argon2id with a random salt for each value.

# Why using Argon2id?

Using the recommendations of OWASP (https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html), Argon2 was the winner for hashing passwords in 2015 and is quite resistant to multiple attacks (side-channel and GPU-based attacks).

## Parameters

As for parameters, the defaults are used from the Rust implementation of Argon2:
- using Argon2id (which is a combination of Argon2i and Argon2d)
- using the default parameters (t_cost=2, m_cost=19MB, p_cost=1)
- using a salt of 16 bytes

## Salt

For the creation of the salt, the `rand_core::OsRng` from the `rand` crate is used, which uses the operating system's random number generator.
