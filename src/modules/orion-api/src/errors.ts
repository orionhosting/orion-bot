export class OrionAPIError extends Error {
    public readonly response: Response;
    public readonly status: number;

    public constructor(options: { message?: string; response: Response }) {
        super(options.message);
        this.response = options.response;
        this.status = options.response.status;
    }
}
