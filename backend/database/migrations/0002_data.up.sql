INSERT INTO users (id, email, name, email_verified, verification_code)
VALUES
  ('1', 'admin@admin.com', 'John Doe', true, null),
  ('2', 'user@user.com', 'Jane Doe', false, '12345678');

INSERT INTO applications (id, slug, name, description, website, creator_id)
VALUES
  ('1', 'example', 'Example', 'Example application', 'https://example.com', '1');

INSERT INTO application_passwords (application_id, user_id, password)
VALUES
  ('1', '1', '$argon2id$v=19$m=19456,t=2,p=1$Yj2BhHw+jXGLJ0+3t+Gelg$Y9ImJc/04MlVVIwUM1QS9sszg2Ew+BuvLiKMwtffnSI'),
  ('1', '2', '$argon2id$v=19$m=19456,t=2,p=1$Yj2BhHw+jXGLJ0+3t+Gelg$Y9ImJc/04MlVVIwUM1QS9sszg2Ew+BuvLiKMwtffnSI'); --- Password: admin
