-- noinspection SqlNoDataSourceInspectionForFile

CREATE TABLE users (
    id VARCHAR(64) NOT NULL,
    refresh_token VARCHAR(128) NOT NULL,
    primary key (id)
);

CREATE TABLE api_tokens (
    token VARCHAR(32) NOT NULL,
    name VARCHAR(64) NOT NULL,
    expiry BIGINT DEFAULT NULL,
    primary key (token)
);

CREATE TABLE oauth2_login_states (
    state VARCHAR(32) NOT NULL,
    token VARCHAR(32) NOT NULL,
    primary key (state),
    foreign key (token) references api_tokens(token)
);

CREATE TABLE oauth2_access_tokens (
    token VARCHAR(64) NOT NULL,
    user VARCHAR(32) NOT NULL,
    expiry BIGINT NOT NULL,
    primary key (token),
    foreign key (user) references users(id)
);