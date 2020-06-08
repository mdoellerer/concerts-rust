CREATE SEQUENCE artists_id_seq;

CREATE TABLE public.artists
(
    id bigint NOT NULL DEFAULT nextval('artists_id_seq'::regclass),
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    country character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT artists_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.artists
    OWNER to "sou.admin";

ALTER SEQUENCE artists_id_seq
    OWNED BY artists.id;
