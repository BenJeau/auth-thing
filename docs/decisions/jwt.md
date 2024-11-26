# JWT

All JWT configuration are setup per application, a single application has a main configuration, and can have many configurations (which includes, algorithm, public key, private key, etc).

## Recommendations

- Use ES256/EdDSA algorithm, an elliptic curve algorithm, which also uses asymmetric keys
- Asymmetric keys are used to sign and verify the token
  - Public key is used to verify the signature
  - Private key is used to sign the token

## Implementation

- All configurations + keys are stored in the database, and can be rotated
- Private keys are encrypted before stored in the database
- A JWK Set is generated for each application, and served at the /auth/application/{applicationSlug}/.well-known/jwks.json endpoint
