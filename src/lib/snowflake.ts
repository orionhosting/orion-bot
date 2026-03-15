import { Snowflake } from "@sodiumlabs/snowflake";

/**
 * A class for parsing snowflake ids using Voctal's snowflake epoch
 *
 * Which is 2020-01-01 at 00:00:00.000 UTC+0
 */
export const snowflake = new Snowflake(1609459200000);
