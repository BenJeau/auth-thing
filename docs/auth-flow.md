Flows:
- login
- signup
- reset password
- email verification
- update password
  - by user request
  - by admin request
  - by expire of password
  - by new password policy
  - by new application access with differing password policy
- OTP verification
- OTP setup
- OTP backup codes
- disable OTP
- OpenID connect
- passwordless
- request role change
- major changes email notifications (+ login with diff location/ip)

# Login

## Email/Password

- Ask for email and password
- Validate email and password on server and client
  - Since password requirements change per application, we return requirements from the server to have same validation on client
  - We use zxcvbn to validate password strength
  - 

## OpenID Connect