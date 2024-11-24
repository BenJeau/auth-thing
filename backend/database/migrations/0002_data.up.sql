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
