import App from './app/App.js';

if (window && "document" in window) {
    window.document.addEventListener("DOMContentLoaded", () => {
        console.log('DOM fully loaded and parsed');
    });
}
