PGDMP  
    +                |            db_belajar_actix    16.0    16.0     �           0    0    ENCODING    ENCODING        SET client_encoding = 'UTF8';
                      false            �           0    0 
   STDSTRINGS 
   STDSTRINGS     (   SET standard_conforming_strings = 'on';
                      false            �           0    0 
   SEARCHPATH 
   SEARCHPATH     8   SELECT pg_catalog.set_config('search_path', '', false);
                      false            �           1262    58496    db_belajar_actix    DATABASE     �   CREATE DATABASE db_belajar_actix WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE_PROVIDER = libc LOCALE = 'English_Indonesia.1252';
     DROP DATABASE db_belajar_actix;
                postgres    false            �            1259    58497    tb_users    TABLE     �   CREATE TABLE public.tb_users (
    id character varying(36) NOT NULL,
    user_code character varying(15),
    username character varying(50),
    password character varying(255),
    email text
);
    DROP TABLE public.tb_users;
       public         heap    postgres    false            �          0    58497    tb_users 
   TABLE DATA           L   COPY public.tb_users (id, user_code, username, password, email) FROM stdin;
    public          postgres    false    215   �       P           2606    58503    tb_users tb_users_pkey 
   CONSTRAINT     T   ALTER TABLE ONLY public.tb_users
    ADD CONSTRAINT tb_users_pkey PRIMARY KEY (id);
 @   ALTER TABLE ONLY public.tb_users DROP CONSTRAINT tb_users_pkey;
       public            postgres    false    215            �      x������ � �     