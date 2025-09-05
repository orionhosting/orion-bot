import express from "express";

export default () => {
    const app = express();

    app.get("/ping", (_, res) => {
        res.sendStatus(200);
    });

    app.all("/*notfound", (_, res) => {
        res.sendStatus(404);
    });

    return app;
};
