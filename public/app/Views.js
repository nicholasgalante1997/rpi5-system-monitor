class ViewEngine {
  #views = new Map();

  constructor() {
    this.#views.set('home', this.#renderHomeView.bind(this));
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
    const markdown = await this.#fetchView('home.md');
    const content = markdown.asHtml();
    const postsList = await this.#posts.fetchPosts();
    const postsMarkup = this.#transformPostsToList(postsList.slice(0, 3));

    if (this.#appContainer) {
      this.#appContainer.innerHTML = `
        <div class="markdown-content">
          ${content}
        </div>
        <h2 class="recent-posts-label">Recent Posts</h2>
        <div class="post-list" id="recent-posts">
          ${postsMarkup}
        </div>
      `;
    }
  }

  async #renderErrorView(error) {}

  async #fetchView(view) {
    const url = `/api/http-views/pages/${view}`;
    const config = { method: 'GET', headers: { 'Accept': 'text/html', 'Accept-Encoding': 'gzip, deflate, br' } };
    const response = await fetch(url);
  }
}

export default ViewEngine;
