import dompurify from 'dompurify';
import Config from './Config.js';

class ViewEngine {
  #views = new Map();

  constructor() {
    this.#views.set('home', this.#renderHomeView.bind(this));
    this.#views.set('cpu', this.#renderCpuView.bind(this));
    this.#views.set('memory', this.#renderMemoryView.bind(this));
    this.#views.set('disks', this.#renderDisksView.bind(this));
    this.#views.set('components', this.#renderComponentsView.bind(this));
    this.#views.set('404', this.#render404View.bind(this));
  }

  async render(view, params = {}) {
    try {
      const render = this.#views.get(view);
      if (render && typeof render === 'function') {
        await Promise.resolve(render(params));
      } else {
        console.error(`View "${view}" not found`);
        throw new Error('View "' + view + '" not found');
      }
    } catch (e) {
      console.error(e);
      this.#renderErrorView(e);
    }
  }

  async #renderHomeView() {
    try {
      const html = await this.#fetchView('overview');
      const app = document.getElementById(Config.app.elementId);
      app.innerHTML = html;
    } catch (error) {
      console.error('Error fetching home view:', error);
      throw new Error('Error fetching home view');
    }
  }

  async #renderCpuView() {
    try {
      const html = await this.#fetchView('cpu');
      const app = document.getElementById(Config.app.elementId);
      app.innerHTML = html;
    } catch (error) {
      console.error('Error fetching CPU view:', error);
      throw new Error('Error fetching CPU view');
    }
  }

  async #renderMemoryView() {
    try {
      const html = await this.#fetchView('memory');
      const app = document.getElementById(Config.app.elementId);
      app.innerHTML = html;
    } catch (error) {
      console.error('Error fetching memory view:', error);
      throw new Error('Error fetching memory view');
    }
  }

  async #renderDisksView() {
    try {
      const html = await this.#fetchView('disks');
      const app = document.getElementById(Config.app.elementId);
      app.innerHTML = html;
    } catch (error) {
      console.error('Error fetching memory view:', error);
      throw new Error('Error fetching memory view');
    }
  }

  async #renderComponentsView() {
    try {
      const html = await this.#fetchView('components');
      const app = document.getElementById(Config.app.elementId);
      app.innerHTML = html;
    } catch (error) {
      console.error('Error fetching components view:', error);
      throw new Error('Error fetching components view');
    }
  }

  async #renderErrorView(error) {
    const app = document.getElementById(Config.app.elementId);
    app.innerHTML = `
      <h1>Error</h1>
      <p>${error.message}</p>
      <pre>${error.stack}</pre>
    `;
    console.error('Error:', error);
    console.error(error.stack);
  }

  async #render404View() {
    const app = document.getElementById(Config.app.elementId);
    app.innerHTML = '<h1>404 - Page Not Found</h1>';
  }

  async #fetchView(view) {
    const url = `/api/http-views/pages/${view}`;
    const config = { method: 'GET', headers: { 'Accept': 'text/html', 'Accept-Encoding': 'gzip, deflate, br' } };
    const response = await fetch(url, config);
    const html = await response.text();
    return dompurify.sanitize(html);
  }
}

export default ViewEngine;
