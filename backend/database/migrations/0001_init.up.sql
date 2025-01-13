CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    username TEXT,
    picture TEXT,
    disabled BOOLEAN NOT NULL DEFAULT FALSE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_code TEXT,
    verification_code_created_at DATETIME,
    preferred_locale TEXT,
    two_factor_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    two_factor_secret TEXT
);

CREATE TABLE applications (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    creator_id INTEGER NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    website TEXT, -- unsure if this is needed...
    icon TEXT,
    api_token_auth_enabled BOOLEAN DEFAULT FALSE NOT NULL,
    basic_auth_enabled BOOLEAN DEFAULT FALSE NOT NULL,
    password_auth_enabled BOOLEAN DEFAULT FALSE NOT NULL,
    password_auth_signup_enabled BOOLEAN DEFAULT FALSE NOT NULL,
    password_min_length INTEGER DEFAULT 0 NOT NULL,
    password_max_length INTEGER DEFAULT 2147483647 NOT NULL, -- 2^31 - 1, max value for INTEGER
    password_min_lowercase INTEGER DEFAULT 0 NOT NULL,
    password_min_uppercase INTEGER DEFAULT 0 NOT NULL,
    password_min_number INTEGER DEFAULT 0 NOT NULL,
    password_min_special INTEGER DEFAULT 0 NOT NULL,
    password_unique BOOLEAN DEFAULT FALSE NOT NULL,
    password_min_strength TEXT NOT NULL DEFAULT 'weak'
        CHECK (password_min_strength IN ('weak', 'medium', 'strong')),
    verification_required BOOLEAN DEFAULT FALSE NOT NULL,
    verification_method TEXT,
    active_jwt_config_id INTEGER,
    FOREIGN KEY (creator_id) REFERENCES users(id),
    FOREIGN KEY (active_jwt_config_id) REFERENCES jwt_configs(id)
);

CREATE TABLE jwt_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    algorithm TEXT NOT NULL
        CHECK (algorithm IN ('HS256', 'HS384', 'HS512', 'RS256', 'RS384', 'RS512', 'ES256', 'ES384', 'EdDSA')),
    private_key BLOB NOT NULL,
    public_key BLOB NOT NULL,
    audience BLOB NOT NULL,
    expiration INTEGER NOT NULL,
    FOREIGN KEY (application_id) REFERENCES applications(id) ON DELETE CASCADE
);

CREATE TABLE application_passwords (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    password TEXT NOT NULL,
    FOREIGN KEY (application_id) REFERENCES applications(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
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
    FOREIGN KEY (application_id) REFERENCES applications(id) ON DELETE CASCADE
);

CREATE TABLE users_roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id INTEGER NOT NULL,
    role_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
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
    auth_url TEXT NOT NULL,
    UNIQUE (name, kind)
);

CREATE TABLE application_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    provider_id INTEGER NOT NULL,
    FOREIGN KEY (application_id) REFERENCES applications(id) ON DELETE CASCADE,
    FOREIGN KEY (provider_id) REFERENCES providers(id) ON DELETE CASCADE
);

CREATE TABLE action_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id INTEGER NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT NOT NULL,
    uri TEXT NOT NULL,
    method TEXT NOT NULL
);

CREATE TABLE api_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    application_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    token TEXT NOT NULL,
    note TEXT NOT NULL,
    expires_at DATETIME,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (application_id) REFERENCES applications(id) ON DELETE CASCADE
);