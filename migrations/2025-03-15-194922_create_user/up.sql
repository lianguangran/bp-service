-- Your SQL goes here
CREATE TABLE users
(
    id                UUID PRIMARY KEY                  default uuid_generate_v4(),
    openid            VARCHAR                  NOT NULL,
    session_key       VARCHAR                  NOT NULL,
    created_at        TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    updated_at        TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp
);

create unique index idx_users_openid on users (openid);

comment on table users is '用户表';
comment on column users.id is '编号';
comment on column users.openid is '微信编号';
comment on column users.session_key is '微信密钥';
comment on column users.created_at is '创建时间';
comment on column users.updated_at is '更新时间';