import { z } from "zod";
import { envSchema } from "./validation/env";

declare global {
    namespace NodeJS {
        export interface ProcessEnv extends Readonly<z.infer<typeof envSchema>> {}
    }
}

export {};
