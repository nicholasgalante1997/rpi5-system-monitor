import dompurify from 'dompurify';

class HypertextAppEngine {
    async fetchHypertextFromServer(path) {
        const url = `/api/http-views/${path}`;
        const config = { method: 'GET', headers: { 'Accept': 'text/html' } };
        const response = await fetch(url, config);
        const htmlDirty = await response.text();
        const htmlSanitized = dompurify.sanitize(htmlDirty);
        return htmlSanitized;
    }
}