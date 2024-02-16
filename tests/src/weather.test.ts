import supertest from 'supertest';

const BASE_URL = 'http://0.0.0.0:3000';
const request = supertest(BASE_URL);

const SUCCESS_STATUS = 200;
const REDIRECT_STATUS = 308;
const NOT_FOUND_STATUS = 404;
const DEFAULT_CITY = 'Montana';
const NON_EXISTENT_CITY = 'NonExistentCity';

describe('Weather API redirection', () => {
    it(`GET / should redirect to /weather/${DEFAULT_CITY}`, async () => {
        const response = await request.get('/');
        expect(response.status).toBe(REDIRECT_STATUS);
        expect(response.header.location).toBe(`/weather/${DEFAULT_CITY}`);
    });
});

describe('Weather API performance', () => {
    const REQUEST_TIMES = 50;
    const MEAN_LIMIT = 50;

    it(`GET /weather/${DEFAULT_CITY} during high load should still be less than ${MEAN_LIMIT}ms`, async () => {
        const requestTimes: {
            status: number,
            time: number
        }[] = [];

        for (let i = 0; i < REQUEST_TIMES; i++) {
            const start = new Date().getTime();
            const response = await request.get(`/weather/${encodeURIComponent(DEFAULT_CITY)}`);
            const status_code = response.status;
            const end = new Date().getTime();

            requestTimes.push({
                status: status_code,
                time: end - start
            });
        }

        const allRequestsSuccessful = requestTimes.every((request) => request.status === SUCCESS_STATUS);
        const meanRequestTime = requestTimes.reduce((acc, request) => acc + request.time, 0) / REQUEST_TIMES;

        expect(allRequestsSuccessful).toBe(true);
        expect(meanRequestTime).toBeLessThan(MEAN_LIMIT);
    });
});

describe('Weather API edge cases', () => {
    it(`GET /weather/${NON_EXISTENT_CITY} should return ${NOT_FOUND_STATUS}`, async () => {
        const response = await request.get(`/weather/${NON_EXISTENT_CITY}`);
        expect(response.status).toBe(NOT_FOUND_STATUS);
    });

    it('GET /weather/ with no city should return 404', async () => {
        const response = await request.get('/weather');
        expect(response.status).toBe(NOT_FOUND_STATUS);
    });
});