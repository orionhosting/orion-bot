import { Schema, model, type InferSchemaType, type HydratedDocument } from "mongoose";

const schema = new Schema({});

export type ManagerSchemaDefinition = InferSchemaType<typeof schema>;

export type Manager = HydratedDocument<ManagerSchemaDefinition>;

export const ManagerModel = model("Manager", schema);
