CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    email TEXT NOT NULL,
    name TEXT,
    username TEXT,
    picture TEXT,
    disabled BOOLEAN DEFAULT FALSE NOT NULL,
    verified BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE applications (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    creator_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    website TEXT, -- unsure if this is needed...
    icon TEXT,
    password_auth BOOLEAN DEFAULT FALSE NOT NULL,
    password_min_length INTEGER DEFAULT 8 NOT NULL,
    password_max_length INTEGER,
    password_requires_lowercase BOOLEAN DEFAULT FALSE NOT NULL,
    password_requires_uppercase BOOLEAN DEFAULT FALSE NOT NULL,
    password_requires_number BOOLEAN DEFAULT FALSE NOT NULL,
    password_requires_special BOOLEAN DEFAULT FALSE NOT NULL,
    password_requires_unique BOOLEAN DEFAULT FALSE NOT NULL,
    password_requires_non_common BOOLEAN DEFAULT FALSE NOT NULL,
    verification_required BOOLEAN DEFAULT FALSE NOT NULL,
    verification_method TEXT,
    verification_code TEXT,
    FOREIGN KEY (creator_id) REFERENCES users(id)
);

CREATE TABLE application_passwords (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    password TEXT NOT NULL,
    FOREIGN KEY (application_id) REFERENCES applications(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    creator_id INTEGER NOT NULL,
    application_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (creator_id) REFERENCES users(id),
    FOREIGN KEY (application_id) REFERENCES applications(id)
);

CREATE TABLE users_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- CREATE TABLE sessions (
--     id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
--     created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
--     user_id INTEGER NOT NULL,
--     token TEXT NOT NULL,
--     FOREIGN KEY (user_id) REFERENCES users(id)
-- );

CREATE TABLE providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    client_id TEXT NOT NULL,
    client_secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    UNIQUE (name, kind)
);

CREATE TABLE application_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    provider_id INTEGER NOT NULL,
    FOREIGN KEY (application_id) REFERENCES applications(id),
    FOREIGN KEY (provider_id) REFERENCES providers(id)
);