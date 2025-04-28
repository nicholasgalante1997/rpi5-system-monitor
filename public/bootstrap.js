import App from './app/App.js';

if (window && "document" in window) {
    window.document.addEventListener("DOMContentLoaded", () => {
        window.AppInstance = new App();
    });
}
