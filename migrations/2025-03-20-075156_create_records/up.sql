-- Your SQL goes here
CREATE TABLE records
(
    id         UUID PRIMARY KEY                  default uuid_generate_v4(),
    member_id  UUID                     NOT NULL,
    systolic   INT                      NOT NULL,
    diastolic  INT                      NOT NULL,
    bmp        INT                      NOT NULL,
    record_at  TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default current_timestamp
);

create index idx_records_member_id on records (member_id);

comment on table records is '记录表';
comment on column records.id is '编号';
comment on column records.member_id is '成员编号';
comment on column records.systolic is '收缩压';
comment on column records.diastolic is '舒张压';
comment on column records.bmp is '心率';
comment on column records.record_at is '记录时间';
comment on column records.created_at is '创建时间';
comment on column records.updated_at is '更新时间';