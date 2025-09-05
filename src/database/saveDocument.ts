import type { HydratedDocument } from "mongoose";

const saved = new Set<unknown>();
const needSave = new Set<unknown>();

export const saveDocument = async <I>(document: HydratedDocument<I>): Promise<boolean> => {
    const id = document._id;

    if (saved.has(id)) {
        needSave.add(id);
        return true;
    }

    saved.add(id);
    const newManager = await document.save();

    process.nextTick(async () => {
        saved.delete(id);

        if (needSave.has(id)) {
            needSave.delete(id);
            await saveDocument(document);
        }
    });

    return newManager === document;
};
