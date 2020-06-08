CREATE SEQUENCE venues_id_seq;

CREATE TABLE public.venues
(
    id bigint NOT NULL DEFAULT nextval('venues_id_seq'::regclass),
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    city character varying(255) COLLATE pg_catalog."default" NOT NULL,
    country character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT venues_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.venues
    OWNER to "sou.admin";

ALTER SEQUENCE venues_id_seq
    OWNED BY venues.id;
