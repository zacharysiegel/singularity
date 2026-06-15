\restrict dbmate

-- Dumped from database version 18.0
-- Dumped by pg_dump version 18.4 (Homebrew)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: accolade; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.accolade (
    id uuid DEFAULT uuidv7() NOT NULL,
    account_id uuid NOT NULL,
    game_id uuid NOT NULL,
    accolade_type text NOT NULL,
    awarded timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: account; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.account (
    id uuid DEFAULT uuidv7() NOT NULL,
    email text NOT NULL,
    username text NOT NULL,
    password_hash text NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL,
    updated timestamp with time zone DEFAULT now() NOT NULL,
    deleted_at timestamp with time zone
);


--
-- Name: COLUMN account.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.account.deleted_at IS 'NULL indicates active account; non-NULL indicates soft-deleted with PII anonymized';


--
-- Name: conversation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.conversation (
    id uuid DEFAULT uuidv7() NOT NULL,
    game_id uuid,
    name text,
    created timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN conversation.game_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.conversation.game_id IS 'NULL for global conversations, non-NULL for in-game conversations';


--
-- Name: conversation_message; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.conversation_message (
    id uuid DEFAULT uuidv7() NOT NULL,
    conversation_id uuid NOT NULL,
    sender_account_id uuid NOT NULL,
    content text NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: conversation_latest_message_view; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.conversation_latest_message_view AS
 SELECT conversation_id,
    max(created) AS latest_message_created
   FROM public.conversation_message
  GROUP BY conversation_id;


--
-- Name: VIEW conversation_latest_message_view; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.conversation_latest_message_view IS 'Most recent message timestamp per conversation, for sorting conversation lists';


--
-- Name: conversation_member; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.conversation_member (
    conversation_id uuid NOT NULL,
    account_id uuid NOT NULL,
    entered timestamp with time zone DEFAULT now() NOT NULL,
    exited timestamp with time zone
);


--
-- Name: COLUMN conversation_member.exited; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.conversation_member.exited IS 'NULL indicates active membership';


--
-- Name: follow; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.follow (
    source_account_id uuid NOT NULL,
    target_account_id uuid NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL,
    CONSTRAINT follow_check CHECK ((source_account_id <> target_account_id))
);


--
-- Name: game; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.game (
    id uuid DEFAULT uuidv7() NOT NULL,
    name text NOT NULL,
    creator_id uuid NOT NULL,
    status integer DEFAULT 0 NOT NULL,
    max_players integer DEFAULT 8 NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL,
    updated timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN game.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.game.status IS 'Integer enum: 0=Pending, 1=Active, 2=Completed (see shared::schema::game::GameStatus)';


--
-- Name: game_membership; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.game_membership (
    game_id uuid NOT NULL,
    account_id uuid NOT NULL,
    joined timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: game_member_count_view; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.game_member_count_view AS
 SELECT game.id AS game_id,
    count(game_membership.game_id) AS member_count
   FROM (public.game
     LEFT JOIN public.game_membership ON ((game_membership.game_id = game.id)))
  GROUP BY game.id;


--
-- Name: VIEW game_member_count_view; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.game_member_count_view IS 'Member count per game including zero-member games via left join';


--
-- Name: game_session; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.game_session (
    id uuid DEFAULT uuidv7() NOT NULL,
    game_id uuid NOT NULL,
    account_id uuid NOT NULL,
    session_id uuid NOT NULL,
    entered timestamp with time zone DEFAULT now() NOT NULL,
    exited timestamp with time zone
);


--
-- Name: COLUMN game_session.exited; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.game_session.exited IS 'NULL indicates active session; non-NULL indicates the player has disconnected from the game';


--
-- Name: mutual_follow_view; Type: VIEW; Schema: public; Owner: -
--

CREATE VIEW public.mutual_follow_view AS
 SELECT f1.source_account_id AS account_id,
    f1.target_account_id AS mutual_account_id
   FROM (public.follow f1
     JOIN public.follow f2 ON (((f1.source_account_id = f2.target_account_id) AND (f1.target_account_id = f2.source_account_id))));


--
-- Name: VIEW mutual_follow_view; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON VIEW public.mutual_follow_view IS 'Accounts that follow each other. Not materialized; both sides hit the composite PK index.';


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying NOT NULL
);


--
-- Name: session; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.session (
    id uuid DEFAULT uuidv7() NOT NULL,
    account_id uuid NOT NULL,
    token text NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL,
    expiry timestamp with time zone CONSTRAINT session_expires_not_null NOT NULL
);


--
-- Name: COLUMN session.expiry; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.session.expiry IS 'Session is rejected when expiry < now(); extended via debounced sliding window on authenticated requests';


--
-- Name: statistic; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.statistic (
    id uuid DEFAULT uuidv7() NOT NULL,
    account_id uuid NOT NULL,
    game_id uuid,
    statistic_type text NOT NULL,
    value double precision DEFAULT 0 NOT NULL,
    updated timestamp with time zone DEFAULT now() NOT NULL,
    created timestamp with time zone DEFAULT now() NOT NULL
);


--
-- Name: accolade accolade_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.accolade
    ADD CONSTRAINT accolade_pkey PRIMARY KEY (id);


--
-- Name: account account_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_email_key UNIQUE (email);


--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY (id);


--
-- Name: account account_username_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_username_key UNIQUE (username);


--
-- Name: conversation_member conversation_member_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_member
    ADD CONSTRAINT conversation_member_pkey PRIMARY KEY (conversation_id, account_id);


--
-- Name: conversation_message conversation_message_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_message
    ADD CONSTRAINT conversation_message_pkey PRIMARY KEY (id);


--
-- Name: conversation conversation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation
    ADD CONSTRAINT conversation_pkey PRIMARY KEY (id);


--
-- Name: follow follow_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.follow
    ADD CONSTRAINT follow_pkey PRIMARY KEY (source_account_id, target_account_id);


--
-- Name: game_membership game_membership_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_membership
    ADD CONSTRAINT game_membership_pkey PRIMARY KEY (game_id, account_id);


--
-- Name: game game_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game
    ADD CONSTRAINT game_pkey PRIMARY KEY (id);


--
-- Name: game_session game_session_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_session
    ADD CONSTRAINT game_session_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: session session_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_pkey PRIMARY KEY (id);


--
-- Name: session session_token_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_token_key UNIQUE (token);


--
-- Name: statistic statistic_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.statistic
    ADD CONSTRAINT statistic_pkey PRIMARY KEY (id);


--
-- Name: idx_accolade_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_accolade_account_id ON public.accolade USING btree (account_id);


--
-- Name: idx_accolade_game_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_accolade_game_id ON public.accolade USING btree (game_id);


--
-- Name: idx_conversation_game_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_conversation_game_id ON public.conversation USING btree (game_id);


--
-- Name: idx_conversation_message_conversation_created; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_conversation_message_conversation_created ON public.conversation_message USING btree (conversation_id, created);


--
-- Name: INDEX idx_conversation_message_conversation_created; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_conversation_message_conversation_created IS 'Messages in a conversation sorted by time';


--
-- Name: idx_conversation_message_sender_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_conversation_message_sender_account_id ON public.conversation_message USING btree (sender_account_id);


--
-- Name: idx_follow_target; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_follow_target ON public.follow USING btree (target_account_id);


--
-- Name: INDEX idx_follow_target; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_follow_target IS 'Reverse lookup; the composite PK already covers (source_account_id, target_account_id)';


--
-- Name: idx_game_creator_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_game_creator_id ON public.game USING btree (creator_id);


--
-- Name: idx_game_session_active; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX idx_game_session_active ON public.game_session USING btree (game_id, account_id) WHERE (exited IS NULL);


--
-- Name: INDEX idx_game_session_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON INDEX public.idx_game_session_active IS 'Partial unique index preventing duplicate active sessions per player per game';


--
-- Name: idx_game_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_game_status ON public.game USING btree (status);


--
-- Name: idx_session_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_session_account_id ON public.session USING btree (account_id);


--
-- Name: idx_session_token; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_session_token ON public.session USING btree (token);


--
-- Name: idx_statistic_account_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_statistic_account_id ON public.statistic USING btree (account_id);


--
-- Name: idx_statistic_game_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_statistic_game_id ON public.statistic USING btree (game_id);


--
-- Name: accolade accolade_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.accolade
    ADD CONSTRAINT accolade_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: accolade accolade_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.accolade
    ADD CONSTRAINT accolade_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- Name: conversation conversation_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation
    ADD CONSTRAINT conversation_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- Name: conversation_member conversation_member_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_member
    ADD CONSTRAINT conversation_member_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: conversation_member conversation_member_conversation_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_member
    ADD CONSTRAINT conversation_member_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES public.conversation(id) ON DELETE CASCADE;


--
-- Name: conversation_message conversation_message_conversation_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_message
    ADD CONSTRAINT conversation_message_conversation_id_fkey FOREIGN KEY (conversation_id) REFERENCES public.conversation(id) ON DELETE CASCADE;


--
-- Name: conversation_message conversation_message_sender_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.conversation_message
    ADD CONSTRAINT conversation_message_sender_account_id_fkey FOREIGN KEY (sender_account_id) REFERENCES public.account(id);


--
-- Name: follow follow_source_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.follow
    ADD CONSTRAINT follow_source_account_id_fkey FOREIGN KEY (source_account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: follow follow_target_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.follow
    ADD CONSTRAINT follow_target_account_id_fkey FOREIGN KEY (target_account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: game game_creator_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game
    ADD CONSTRAINT game_creator_id_fkey FOREIGN KEY (creator_id) REFERENCES public.account(id);


--
-- Name: game_membership game_membership_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_membership
    ADD CONSTRAINT game_membership_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: game_membership game_membership_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_membership
    ADD CONSTRAINT game_membership_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- Name: game_session game_session_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_session
    ADD CONSTRAINT game_session_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: game_session game_session_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_session
    ADD CONSTRAINT game_session_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- Name: game_session game_session_session_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.game_session
    ADD CONSTRAINT game_session_session_id_fkey FOREIGN KEY (session_id) REFERENCES public.session(id) ON DELETE CASCADE;


--
-- Name: session session_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: statistic statistic_account_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.statistic
    ADD CONSTRAINT statistic_account_id_fkey FOREIGN KEY (account_id) REFERENCES public.account(id) ON DELETE CASCADE;


--
-- Name: statistic statistic_game_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.statistic
    ADD CONSTRAINT statistic_game_id_fkey FOREIGN KEY (game_id) REFERENCES public.game(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict dbmate


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20251109215340'),
    ('20260412000000'),
    ('20260412000001'),
    ('20260412000002'),
    ('20260412000003'),
    ('20260412000004'),
    ('20260412000005'),
    ('20260415000000'),
    ('20260416000000'),
    ('20260416000001'),
    ('20260422000000'),
    ('20260422000001'),
    ('20260423000000'),
    ('20260423000001'),
    ('20260423000002'),
    ('20260424000000');
