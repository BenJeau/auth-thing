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
  ('1', 'EdDSA', 
   X'705b08ffb22b5d1284e8a345393c2b99a70dc28fbfd8bf90d058e834884fa6c4a1dda3b117de73ba846d17c06c3116b271416248a3a900623f053fc1dff45a52a44980a5e2d3c5121aaaf2a002f7f3b005bfbd51e636ac022296a77d61e6cbcd96d9d26d14a6d4a989c499cd64b795e4921cb97575c5a7a2f3c85e',
   X'2f328ee00414be3a2817593552a25fb7a18056158f3bb515bd704fddd20f2886',
   '["http://localhost:5173"]', 
   3600);

UPDATE applications SET active_jwt_config_id = 1 WHERE id = 1;
