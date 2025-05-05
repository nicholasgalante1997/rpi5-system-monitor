import Config from './Config.js';
import Router from './Router.js';

class App {
  router = new Router(Config.routes);

  constructor() {
    this.init();
  }

  init() {
    this.router.router();
    this.setupEventListeners();
  }

  setupEventListeners() {
    this.setupLinkEventListeners();
    this.setupPopStateEventListeners();
  }

  setupLinkEventListeners() {
    document.addEventListener("click", (e) => {
      const link = e.target.closest("[data-link]");
      if (link) {
        e.preventDefault();
        this.router.push(link.href);
        window.scrollTo({
          top: 0,
          behavior: "smooth"
        });
      }
    });
  }

  setupPopStateEventListeners() {
    window.addEventListener("popstate", () => {
      this.router.router();
    })
  }
}

export default App;
