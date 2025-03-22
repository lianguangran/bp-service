-- Your SQL goes here
CREATE TABLE user_member
(
    user_id    UUID                     NOT NULL,
    member_id  UUID                     NOT NULL,
    is_default BOOLEAN                  NOT NULL default false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    primary key (user_id, member_id)
);

comment on table user_member is '用户成员表';
comment on column user_member.user_id is '用户编号';
comment on column user_member.member_id is '成员编号';
comment on column user_member.is_default is '是否默认成员';
comment on column user_member.created_at is '创建时间';
comment on column user_member.updated_at is '更新时间';