import dompurify from 'dompurify';

class HypertextAppEngine {

    /** 
     * Fetches hypertext from a server and sanitizes it using DOMPurify.
     * 
     * @param {string} path The path of the hypertext to fetch.
     * @returns {Promise<string>} The sanitized hypertext. 
     */
    async fetchHypertextFromServer(path) {
        const url = `/api/http-views/${path}`;
        const config = { method: 'GET', headers: { 'Accept': 'text/html' } };
        const response = await fetch(url, config);
        const html = await response.text();
        return dompurify.sanitize(html);
    }
}

export default new HypertextAppEngine();