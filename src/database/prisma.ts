import { PrismaClient } from "@prisma-generated/client";
import { PrismaBetterSqlite3 } from "@prisma/adapter-better-sqlite3";

const adapter = new PrismaBetterSqlite3({ url: "file:./db.sqlite" });
const prisma = new PrismaClient({ adapter });

export { prisma };
