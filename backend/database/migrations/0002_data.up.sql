INSERT INTO users (id, email, name, email_verified, verification_code, verification_code_created_at)
VALUES
  ('1', 'admin@admin.com', 'John Doe', true, null, null),
  ('2', 'user@user.com', 'Jane Doe', false, '$argon2id$v=19$m=19456,t=2,p=1$ylgx6jpeHPpbgGaq0UT/AQ$/NVMxZXJX6O+SFNZEt78i5FJYlvX2i7yElOvx4aRdwo', DATETIME('now')); --- Code: 12345678

INSERT INTO applications (id, slug, name, description, website, creator_id, password_min_length, password_min_strength)
VALUES
  ('1', 'example', 'Example', 'Example application', 'https://example.com', '1', 8, 'medium');

INSERT INTO application_passwords (application_id, user_id, password)
VALUES
  ('1', '1', '$argon2id$v=19$m=19456,t=2,p=1$Yj2BhHw+jXGLJ0+3t+Gelg$Y9ImJc/04MlVVIwUM1QS9sszg2Ew+BuvLiKMwtffnSI'), --- Password: admin
  ('1', '2', '$argon2id$v=19$m=19456,t=2,p=1$WEOFrGChK0K/BOeP5veVvQ$ZL6Z1KzExNMraCr9SvFSckoBWWVv2e+h3o26yvmNckc'); --- Password: user

INSERT INTO jwt_configs (application_id, algorithm, private_key, public_key, audience, expiration)
VALUES
  ('1', 'ES256', 
   X'e29c45fc43d195b6168a48a09bb3ecc35d80d2f52592db33facd0ea9aa210722c9afb3e08af5f1356209548568258737c7325a905c79b1ba4beed3bf21a01244f7dc6d74fa884fa83fe88f642f2c0f34e1b0d8a4e233fc990f37aadceaf448333dd618840c5eb05f03aa070e8b875107d44b5f00b64b528f37821bea717923b18e2206071f205565e296e07238a8e1e8b627a193620b2f5e3b254580a1a7abac56bf9bdbc4d675a2f037dca5c65666feb4cd',
   X'5d61b66399ec79d7c2419e4c3419053b24869d7d3339273174afed196e05cfcde9bf81594d1ea1d611003677ac81d0c5c234b444e31521f237910e5d1f68e067452e0dbeac64a77736cb75aab8a7bee8f95b282b5899f795eb35e0b9d4426cd6bd3da506e7e3a6c330',
   '["http://localhost:5173"]', 
   3600);

UPDATE applications SET active_jwt_config_id = 1 WHERE id = 1;
