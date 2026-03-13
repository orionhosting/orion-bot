-- CreateTable
CREATE TABLE "GuildSettings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "ad_enabled" BOOLEAN NOT NULL,
    "ad_channel_id" TEXT,
    "ad_message_id" TEXT
);

-- CreateTable
CREATE TABLE "GuildAdState" (
    "guild_id" TEXT NOT NULL PRIMARY KEY,
    "is_valid" BOOLEAN NOT NULL,
    "valid_since" INTEGER,
    "updated_at" INTEGER NOT NULL,
    "last_reward_at" INTEGER
);

-- CreateTable
CREATE TABLE "UserBoostState" (
    "user_id" TEXT NOT NULL PRIMARY KEY,
    "is_boosting" BOOLEAN NOT NULL,
    "boosting_since" INTEGER,
    "updated_at" INTEGER NOT NULL,
    "last_reward_at" INTEGER NOT NULL
);
