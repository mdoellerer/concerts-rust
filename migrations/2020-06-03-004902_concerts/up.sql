CREATE SEQUENCE concerts_id_seq;

CREATE TABLE public.concerts
(
    id bigint NOT NULL DEFAULT nextval('concerts_id_seq'::regclass),
    concert_date date NOT NULL,
    setlist text COLLATE pg_catalog."default",
    created_at timestamp(0) without time zone,
    updated_at timestamp(0) without time zone,
    concert_type_id integer NOT NULL,
    artist_id bigint NOT NULL,
    venue_id bigint NOT NULL,
    CONSTRAINT concerts_pkey PRIMARY KEY (id),
    CONSTRAINT concerts_artist_id_foreign FOREIGN KEY (artist_id)
        REFERENCES public.artists (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT concerts_concert_type_id_foreign FOREIGN KEY (concert_type_id)
        REFERENCES public.concert_types (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT concerts_venue_id_foreign FOREIGN KEY (venue_id)
        REFERENCES public.venues (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE public.concerts
    OWNER to "sou.admin";

ALTER SEQUENCE concerts_id_seq
    OWNED BY concerts.id;
