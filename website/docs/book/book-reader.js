// Book Reader Interactive Features

document.addEventListener('DOMContentLoaded', function () {
  initSidebar();
  initInstallTabs();
  initChapterHighlighting();
  initReadingProgress();
});

function initSidebar() {
  const toggleBtn = document.getElementById('toggleSidebar');
  const sidebar = document.getElementById('bookSidebar');

  if (toggleBtn && sidebar) {
    toggleBtn.addEventListener('click', function () {
      sidebar.classList.toggle('open');
      this.textContent = sidebar.classList.contains('open') ? '✕ Close' : '☰ Chapters';
    });

    // Close sidebar when clicking outside on mobile
    document.addEventListener('click', function (e) {
      if (window.innerWidth <= 1024 &&
          sidebar.classList.contains('open') &&
          !sidebar.contains(e.target) &&
          !toggleBtn.contains(e.target)) {
        sidebar.classList.remove('open');
        toggleBtn.textContent = '☰ Chapters';
      }
    });
  }

  // Sidebar link clicks — close on mobile
  document.querySelectorAll('.sidebar-link').forEach(link => {
    link.addEventListener('click', function () {
      if (window.innerWidth <= 1024 && sidebar) {
        sidebar.classList.remove('open');
        if (toggleBtn) toggleBtn.textContent = '☰ Chapters';
      }
    });
  });
}

function initInstallTabs() {
  const tabs = document.querySelectorAll('.itab');
  tabs.forEach(tab => {
    tab.addEventListener('click', function () {
      const os = this.getAttribute('data-os');
      document.querySelectorAll('.itab').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.install-tab-content').forEach(c => c.classList.remove('active'));
      this.classList.add('active');
      const content = document.getElementById('it-' + os);
      if (content) content.classList.add('active');
    });
  });
}

function initChapterHighlighting() {
  const chapters = document.querySelectorAll('.chapter, .book-hero');
  const sidebarLinks = document.querySelectorAll('.sidebar-link');

  const observer = new IntersectionObserver(function (entries) {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        const id = entry.target.id;
        sidebarLinks.forEach(link => {
          link.classList.remove('active');
          if (link.getAttribute('href') === '#' + id) {
            link.classList.add('active');
          }
        });
      }
    });
  }, { threshold: 0.3, rootMargin: '-80px 0px -60% 0px' });

  chapters.forEach(ch => observer.observe(ch));
}

function initReadingProgress() {
  // Add reading progress bar
  const progressBar = document.createElement('div');
  progressBar.style.cssText = `
    position: fixed;
    top: 60px;
    left: 0;
    height: 3px;
    background: linear-gradient(90deg, #f59e0b, #d97706);
    z-index: 999;
    transition: width 0.1s ease;
    width: 0%;
  `;
  document.body.appendChild(progressBar);

  window.addEventListener('scroll', function () {
    const scrollTop = window.scrollY;
    const docHeight = document.documentElement.scrollHeight - window.innerHeight;
    const progress = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;
    progressBar.style.width = Math.min(progress, 100) + '%';
  });
}
