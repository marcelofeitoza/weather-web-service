import supertest from 'supertest';

const request = supertest('http://0.0.0.0:3000');

describe('Weather API redirection', () => {
    it('GET / should redirect to /weather/São%20Paulo', async () => {
        const response = await request.get('/');
        expect(response.status).toBe(308);
        expect(response.headers.location).toBe("/weather/SÃ£o%20Paulo");
    });
});

describe('Weather API performance', () => {
    it('repeat a request 10 times and check if the mean of the request time of the last four is less than 25ms', async () => {
        const requestTimes: number[] = [];

        for (let i = 0; i < 10; i++) {
            const start = new Date().getTime();
            await request.get('/weather/São%20Paulo');
            const end = new Date().getTime();

            requestTimes.push(end - start);
        }

        const lastFourRequestTimes = requestTimes.slice(1);
        const meanRequestTime = lastFourRequestTimes
            .reduce((a: number, b: number) => a + b, 0) / lastFourRequestTimes.length;

        expect(meanRequestTime).toBeLessThan(25);
    });
});

describe('Weather API edge cases', () => {
    it('GET /weather/NonExistentCity should return 404', async () => {
        const response = await request.get('/weather/NonExistentCity');
        expect(response.status).toBe(404);
    });

    it('GET /weather/ with no city should return 404', async () => {
        const response = await request.get('/weather');
        expect(response.status).toBe(404);
    });

    it('GET /weather/São%20Paulo during high load should still be less than 50ms', async () => {
        const requestTimes: number[] = [];

        for (let i = 0; i < 50; i++) {
            const start = new Date().getTime();
            await request.get('/weather/São%20Paulo');
            const end = new Date().getTime();

            requestTimes.push(end - start);
        }

        const lastFourRequestTimes = requestTimes.slice(1);
        const meanRequestTime = lastFourRequestTimes.reduce((a, b) => a + b, 0) / lastFourRequestTimes.length;

        expect(meanRequestTime).toBeLessThan(50);
    });
});