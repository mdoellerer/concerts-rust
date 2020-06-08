CREATE SEQUENCE users_id_seq;

CREATE TABLE public.users
(
    id bigint NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    email character varying(255) COLLATE pg_catalog."default" NOT NULL,
    email_verified_at timestamp(0) without time zone,
    password character varying(255) COLLATE pg_catalog."default" NOT NULL,
    remember_token character varying(100) COLLATE pg_catalog."default",
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    api_token character varying(60) COLLATE pg_catalog."default",
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_api_token_unique UNIQUE (api_token),
    CONSTRAINT users_email_unique UNIQUE (email)
)

TABLESPACE pg_default;

ALTER TABLE public.users
    OWNER to "sou.admin";

ALTER SEQUENCE users_id_seq
OWNED BY users.id;