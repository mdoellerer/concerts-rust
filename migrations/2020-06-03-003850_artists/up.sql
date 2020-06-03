CREATE SEQUENCE artists_id_seq;

CREATE TABLE public.artists
(
    id bigint NOT NULL DEFAULT nextval('artists_id_seq'::regclass),
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    country character varying(255) COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    CONSTRAINT artists_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE public.artists
    OWNER to "sou.admin";

ALTER SEQUENCE artists_id_seq
    OWNED BY artists.id;
