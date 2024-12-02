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
   X'f2cb84fcd65c70867d16ac5e778cfd44a2fdbdadc66936af73785b9861c6eebad543ca993004b9520a6f85dfa98de36e4044e51c59591227929be7cfaa38a9cba62f16f691a019baf3087678091971d1dac7a9480c460c0919aad2afcffd9ac874235df07dc8a344f83aa572043562acec96baf42136fcaf82c8e3a09c0ea56ed150ad804443862475c3608f383ba03c664a529f1138fde288005b30ca89d7d628e8cd15386d2e3ddbced8f959e4609025b6',
   X'53fd12a93e7f0b908ad22cda25cd76c5e176e2002f2d217a48519058092e8d15bc2135104d0d828fb82600b5532ff755b32f632221d0ba2ea6104274edf55d066aba348af6cdb4be7a6c182fbe6d042092c9418fef4e3525cd2fcad0d3408a4b0765598806321695ca',
   '["http://localhost:5173"]', 
   3600);

UPDATE applications SET active_jwt_config_id = 1 WHERE id = 1;
