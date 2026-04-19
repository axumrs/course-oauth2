CREATE TYPE "user_status" AS ENUM (
    'Pending',
    'Active',
    'Freeze'
);

CREATE TABLE IF NOT EXISTS "users" (
    "id" CHAR(20) PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
    "status" user_status NOT NULL DEFAULT 'Pending',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE "application_status" AS ENUM (
    'Pending',
    'Active',
    'Reject'
);

CREATE TABLE IF NOT EXISTS "applications" (
   "id" CHAR(20) PRIMARY KEY,
   "user_id" CHAR(20) NOT NULL,
   "name" TEXT NOT NULL UNIQUE,
   "description" TEXT NOT NULL DEFAULT '',
   "homepage_url" TEXT NOT NULL DEFAULT '',
   "callback_url" TEXT NOT NULL DEFAULT '',
   "status" application_status NOT NULL DEFAULT 'Pending',
   "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
   "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "application_secrets" (
   "id" CHAR(20) PRIMARY KEY,
   "application_id" CHAR(20) NOT NULL,
   "secret" TEXT NOT NULL,
   "nonce" BYTEA NOT NULL,
   "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE "token_kind" AS ENUM (
    'Token',
    'AccessToken',
    'TempAccessToken'
);

CREATE UNLOGGED TABLE IF NOT EXISTS "tokens" (
    "id" CHAR(20) PRIMARY KEY,
    "user_id" CHAR(20) NOT NULL,
    "nonce" BYTEA NOT NULL,
    "token" TEXT NOT NULL,
    "kind" token_kind NOT NULL,
    "application_id" CHAR(20) NOT NULL DEFAULT '',
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "expired_at" TIMESTAMPTZ NOT NULL
);

CREATE TYPE "authorize_scope" AS ENUM (
    'User',
    'ReadUser',
    'UserEmail'
);

CREATE TABLE IF NOT EXISTS "authorizes" (
    "id" CHAR(20) PRIMARY KEY,
    "application_id" CHAR(20) NOT NULL,
    "user_id" CHAR(20) NOT NULL,
    "scope" authorize_scope NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE("application_id", "user_id")
);