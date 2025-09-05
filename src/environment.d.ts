import type { z } from "zod";
import type { envSchema } from "./validation/schemas";

declare global {
    namespace NodeJS {
        export interface ProcessEnv extends Readonly<z.infer<typeof envSchema>> {}
    }
}

export {};
