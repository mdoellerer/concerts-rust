CREATE SEQUENCE concert_types_id_seq;

CREATE TABLE public.concert_types
(
    id bigint NOT NULL DEFAULT nextval('concert_types_id_seq'::regclass),
    description character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT concert_types_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.concert_types
    OWNER to "sou.admin";

ALTER SEQUENCE concert_types_id_seq
    OWNED BY concert_types.id;
