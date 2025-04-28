import Config from './Config.js';
import Router from './Router.js';

class App {
  router = new Router(Config.routes);

  constructor() {
    this.init();
  }

  init() {
    this.setupEventListeners();
  }

  setupEventListeners() {
    this.setupLinkEventListeners();
  }

  setupLinkEventListeners() {
    document.addEventListener("click", (e) => {
      const link = e.target.closest("[data-link]");
      if (link) {
        e.preventDefault();
        // this.#router.push(link.href);
        // Scroller.top();
      }
    });
  }
}

export default App;
