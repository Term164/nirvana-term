create table tickets (
    id INTEGER primary key,
    ticket_key TEXT not null,
    summary TEXT,
    connection_id INTEGER not null references connections (id),
    last_worked_at INTEGER not null default 0,
    unique (ticket_key, connection_id)
);


create index tickets_by_key on tickets (ticket_key);


create index tickets_by_last_worked on tickets (connection_id, last_worked_at desc);


create table slots (
    id INTEGER primary key,
    ticket_id INTEGER not null references tickets (id),
    connection_id INTEGER not null references connections (id),
    note TEXT,
    started_at INTEGER not null,
    stopped_at INTEGER,
    published_at INTEGER,
    check (
        stopped_at is null
        or stopped_at > started_at
    )
);


create unique index one_running_slot on slots (stopped_at)
where
    stopped_at is null;


create index slots_by_ticket on slots (ticket_id, started_at);


create index slots_unpublished on slots (published_at, started_at)
where
    published_at is null;
