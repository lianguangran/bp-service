-- Your SQL goes here
CREATE TABLE members
(
    id         UUID PRIMARY KEY                  default uuid_generate_v4(),
    name       VARCHAR                  NOT NULL,
    memo       VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp
);

comment on table members is '成员表';
comment on column members.id is '编号';
comment on column members.name is '名称';
comment on column members.memo is '备注';
comment on column members.created_at is '创建时间';
comment on column members.updated_at is '更新时间';